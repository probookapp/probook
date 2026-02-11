//! Thermal printer service for ESC/POS receipt printing
//! Supports USB, Network, and serial connections

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrinterError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Write failed: {0}")]
    WriteFailed(String),
    #[error("Printer not found: {0}")]
    PrinterNotFound(String),
    #[error("Serial port error: {0}")]
    SerialPort(String),
    #[error("Network error: {0}")]
    Network(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    USB,      // USB serial port
    Network,  // TCP/IP
    Serial,   // RS-232 serial
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterConfig {
    pub connection_type: ConnectionType,
    pub address: String,  // Port name for USB/Serial, IP:PORT for Network
    pub paper_width: u8,  // 58mm or 80mm
}

/// ESC/POS command constants
pub mod escpos {
    // Initialize printer
    pub const INIT: &[u8] = &[0x1B, 0x40];

    // Text formatting
    pub const BOLD_ON: &[u8] = &[0x1B, 0x45, 0x01];
    pub const BOLD_OFF: &[u8] = &[0x1B, 0x45, 0x00];
    pub const DOUBLE_HEIGHT_ON: &[u8] = &[0x1B, 0x21, 0x10];
    pub const DOUBLE_WIDTH_ON: &[u8] = &[0x1B, 0x21, 0x20];
    pub const DOUBLE_SIZE_ON: &[u8] = &[0x1B, 0x21, 0x30];
    pub const NORMAL_SIZE: &[u8] = &[0x1B, 0x21, 0x00];

    // Alignment
    pub const ALIGN_LEFT: &[u8] = &[0x1B, 0x61, 0x00];
    pub const ALIGN_CENTER: &[u8] = &[0x1B, 0x61, 0x01];
    pub const ALIGN_RIGHT: &[u8] = &[0x1B, 0x61, 0x02];

    // Line spacing
    pub const LINE_SPACING_DEFAULT: &[u8] = &[0x1B, 0x32];
    pub const LINE_SPACING_TIGHT: &[u8] = &[0x1B, 0x33, 0x10];

    // Paper control
    pub const LINE_FEED: &[u8] = &[0x0A];
    pub const FEED_LINES: fn(u8) -> Vec<u8> = |n| vec![0x1B, 0x64, n];
    pub const CUT_PAPER: &[u8] = &[0x1D, 0x56, 0x00]; // Full cut
    pub const CUT_PAPER_PARTIAL: &[u8] = &[0x1D, 0x56, 0x01]; // Partial cut

    // Cash drawer
    pub const OPEN_DRAWER: &[u8] = &[0x1B, 0x70, 0x00, 0x19, 0xFA];

    // Horizontal line (dashes for 80mm paper)
    pub fn horizontal_line(width: u8) -> Vec<u8> {
        let dashes = "-".repeat(width as usize);
        dashes.into_bytes()
    }
}

/// Receipt line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptLine {
    pub designation: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
    pub discount_percent: Option<f64>,
}

/// Receipt payment info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptPayment {
    pub method: String,
    pub amount: f64,
    pub cash_given: Option<f64>,
    pub change: Option<f64>,
}

/// Receipt data for printing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptData {
    // Header
    pub company_name: String,
    pub company_address: Option<String>,
    pub company_phone: Option<String>,

    // Transaction info
    pub ticket_number: String,
    pub date: String,
    pub time: String,
    pub cashier: Option<String>,
    pub register: Option<String>,

    // Items
    pub lines: Vec<ReceiptLine>,

    // Totals
    pub subtotal_ht: f64,
    pub total_vat: f64,
    pub total_ttc: f64,
    pub discount_amount: Option<f64>,
    pub final_amount: f64,

    // Payment
    pub payments: Vec<ReceiptPayment>,

    // Footer
    pub footer_message: Option<String>,
}

/// Printer connection trait
pub trait PrinterConnection {
    fn write(&mut self, data: &[u8]) -> Result<(), PrinterError>;
    fn flush(&mut self) -> Result<(), PrinterError>;
    fn close(&mut self) -> Result<(), PrinterError>;
}

/// USB/Serial printer connection
pub struct SerialPrinter {
    port: Box<dyn serialport::SerialPort>,
}

impl SerialPrinter {
    pub fn new(port_name: &str) -> Result<Self, PrinterError> {
        let port = serialport::new(port_name, 9600)
            .timeout(Duration::from_secs(5))
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .open()
            .map_err(|e| PrinterError::SerialPort(e.to_string()))?;

        Ok(Self { port })
    }
}

impl PrinterConnection for SerialPrinter {
    fn write(&mut self, data: &[u8]) -> Result<(), PrinterError> {
        self.port.write_all(data)
            .map_err(|e| PrinterError::WriteFailed(e.to_string()))
    }

    fn flush(&mut self) -> Result<(), PrinterError> {
        self.port.flush()
            .map_err(|e| PrinterError::WriteFailed(e.to_string()))
    }

    fn close(&mut self) -> Result<(), PrinterError> {
        Ok(()) // Serial port closes on drop
    }
}

/// Network printer connection
pub struct NetworkPrinter {
    stream: TcpStream,
}

impl NetworkPrinter {
    pub fn new(address: &str) -> Result<Self, PrinterError> {
        let stream = TcpStream::connect_timeout(
            &address.parse().map_err(|e| PrinterError::Network(format!("Invalid address: {}", e)))?,
            Duration::from_secs(5)
        ).map_err(|e| PrinterError::Network(e.to_string()))?;

        stream.set_write_timeout(Some(Duration::from_secs(10)))
            .map_err(|e| PrinterError::Network(e.to_string()))?;

        Ok(Self { stream })
    }
}

impl PrinterConnection for NetworkPrinter {
    fn write(&mut self, data: &[u8]) -> Result<(), PrinterError> {
        self.stream.write_all(data)
            .map_err(|e| PrinterError::WriteFailed(e.to_string()))
    }

    fn flush(&mut self) -> Result<(), PrinterError> {
        self.stream.flush()
            .map_err(|e| PrinterError::WriteFailed(e.to_string()))
    }

    fn close(&mut self) -> Result<(), PrinterError> {
        self.stream.shutdown(std::net::Shutdown::Both)
            .map_err(|e| PrinterError::Network(e.to_string()))
    }
}

/// Receipt builder for ESC/POS commands
pub struct ReceiptBuilder {
    buffer: Vec<u8>,
    #[allow(dead_code)]
    paper_width: u8,
    char_width: u8, // Characters per line based on paper width
}

impl ReceiptBuilder {
    pub fn new(paper_width: u8) -> Self {
        let char_width = match paper_width {
            58 => 32,  // 58mm paper: 32 chars
            80 => 48,  // 80mm paper: 48 chars
            _ => 42,   // Default
        };

        Self {
            buffer: Vec::new(),
            paper_width,
            char_width,
        }
    }

    /// Initialize printer
    pub fn init(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(escpos::INIT);
        self
    }

    /// Center align
    pub fn center(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(escpos::ALIGN_CENTER);
        self
    }

    /// Left align
    pub fn left(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(escpos::ALIGN_LEFT);
        self
    }

    /// Right align
    pub fn right(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(escpos::ALIGN_RIGHT);
        self
    }

    /// Bold on
    pub fn bold(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(escpos::BOLD_ON);
        self
    }

    /// Bold off
    pub fn normal(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(escpos::BOLD_OFF);
        self.buffer.extend_from_slice(escpos::NORMAL_SIZE);
        self
    }

    /// Double size text
    pub fn double_size(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(escpos::DOUBLE_SIZE_ON);
        self
    }

    /// Print text
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.buffer.extend_from_slice(text.as_bytes());
        self
    }

    /// Print text with newline
    pub fn line(&mut self, text: &str) -> &mut Self {
        self.buffer.extend_from_slice(text.as_bytes());
        self.buffer.extend_from_slice(escpos::LINE_FEED);
        self
    }

    /// Print empty line
    pub fn newline(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(escpos::LINE_FEED);
        self
    }

    /// Print multiple empty lines
    pub fn feed(&mut self, lines: u8) -> &mut Self {
        self.buffer.extend_from_slice(&escpos::FEED_LINES(lines));
        self
    }

    /// Print horizontal separator
    pub fn separator(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&escpos::horizontal_line(self.char_width));
        self.buffer.extend_from_slice(escpos::LINE_FEED);
        self
    }

    /// Print two-column line (left and right aligned)
    pub fn two_column(&mut self, left: &str, right: &str) -> &mut Self {
        let left_len = left.chars().count();
        let right_len = right.chars().count();
        let total_width = self.char_width as usize;

        if left_len + right_len >= total_width {
            // Won't fit on one line, print separately
            self.line(left);
            self.right().line(right).left();
        } else {
            let spaces = total_width - left_len - right_len;
            let line = format!("{}{:>width$}", left, right, width = spaces + right_len);
            self.line(&line);
        }
        self
    }

    /// Cut paper
    pub fn cut(&mut self) -> &mut Self {
        self.feed(3);
        self.buffer.extend_from_slice(escpos::CUT_PAPER_PARTIAL);
        self
    }

    /// Open cash drawer
    pub fn open_drawer(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(escpos::OPEN_DRAWER);
        self
    }

    /// Get the built ESC/POS data
    pub fn build(self) -> Vec<u8> {
        self.buffer
    }
}

/// Format currency amount for receipt
fn format_amount(amount: f64, currency: &str) -> String {
    format!("{:.2} {}", amount, currency)
}

/// Build receipt data as ESC/POS commands
pub fn build_receipt(data: &ReceiptData, paper_width: u8, currency: &str) -> Vec<u8> {
    let mut builder = ReceiptBuilder::new(paper_width);

    // Initialize
    builder.init();

    // Header - Company info
    builder.center().bold().double_size();
    builder.line(&data.company_name);
    builder.normal();

    if let Some(addr) = &data.company_address {
        builder.line(addr);
    }
    if let Some(phone) = &data.company_phone {
        builder.line(phone);
    }

    builder.separator();

    // Transaction info
    builder.left();
    builder.two_column("Ticket:", &data.ticket_number);
    builder.two_column("Date:", &format!("{} {}", data.date, data.time));

    if let Some(cashier) = &data.cashier {
        builder.two_column("Caissier:", cashier);
    }
    if let Some(register) = &data.register {
        builder.two_column("Caisse:", register);
    }

    builder.separator();

    // Items
    for line in &data.lines {
        // Product name (may wrap)
        builder.line(&line.designation);

        // Quantity x Price = Total
        let qty_str = if line.quantity.fract() == 0.0 {
            format!("{:.0}", line.quantity)
        } else {
            format!("{:.2}", line.quantity)
        };

        let detail = format!(
            "  {} x {} = {}",
            qty_str,
            format_amount(line.unit_price, currency),
            format_amount(line.total, currency)
        );
        builder.line(&detail);

        // Show discount if any
        if let Some(discount) = line.discount_percent {
            if discount > 0.0 {
                builder.line(&format!("    -{:.0}%", discount));
            }
        }
    }

    builder.separator();

    // Totals
    builder.two_column("Sous-total HT:", &format_amount(data.subtotal_ht, currency));
    builder.two_column("TVA:", &format_amount(data.total_vat, currency));

    if let Some(discount) = data.discount_amount {
        if discount > 0.0 {
            builder.two_column("Remise:", &format!("-{}", format_amount(discount, currency)));
        }
    }

    builder.bold();
    builder.two_column("TOTAL:", &format_amount(data.final_amount, currency));
    builder.normal();

    builder.separator();

    // Payments
    for payment in &data.payments {
        let method_label = match payment.method.as_str() {
            "CASH" => "Especes",
            "CARD" => "Carte",
            _ => &payment.method,
        };
        builder.two_column(method_label, &format_amount(payment.amount, currency));

        if let Some(cash_given) = payment.cash_given {
            builder.two_column("Recu:", &format_amount(cash_given, currency));
        }
        if let Some(change) = payment.change {
            builder.bold();
            builder.two_column("Rendu:", &format_amount(change, currency));
            builder.normal();
        }
    }

    builder.separator();

    // Footer
    builder.center();
    if let Some(footer) = &data.footer_message {
        builder.line(footer);
    } else {
        builder.line("Merci de votre visite!");
    }

    builder.feed(1);
    builder.cut();

    builder.build()
}

/// Print receipt to configured printer
pub fn print_receipt(config: &PrinterConfig, data: &ReceiptData, currency: &str, open_drawer: bool) -> Result<(), PrinterError> {
    let mut receipt_data = build_receipt(data, config.paper_width, currency);

    // Add drawer kick at the start if requested
    if open_drawer {
        let mut with_drawer = escpos::OPEN_DRAWER.to_vec();
        with_drawer.append(&mut receipt_data);
        receipt_data = with_drawer;
    }

    match config.connection_type {
        ConnectionType::USB | ConnectionType::Serial => {
            let mut printer = SerialPrinter::new(&config.address)?;
            printer.write(&receipt_data)?;
            printer.flush()?;
            printer.close()?;
        }
        ConnectionType::Network => {
            let mut printer = NetworkPrinter::new(&config.address)?;
            printer.write(&receipt_data)?;
            printer.flush()?;
            printer.close()?;
        }
    }

    Ok(())
}

/// List available serial ports (for USB printer discovery)
pub fn list_serial_ports() -> Vec<String> {
    serialport::available_ports()
        .map(|ports| {
            ports.into_iter()
                .map(|p| p.port_name)
                .collect()
        })
        .unwrap_or_default()
}

/// Test printer connection
pub fn test_printer(config: &PrinterConfig) -> Result<(), PrinterError> {
    let test_data = {
        let mut builder = ReceiptBuilder::new(config.paper_width);
        builder.init()
            .center()
            .bold()
            .line("=== TEST IMPRESSION ===")
            .normal()
            .line("Imprimante configuree!")
            .feed(2)
            .cut();
        builder.build()
    };

    match config.connection_type {
        ConnectionType::USB | ConnectionType::Serial => {
            let mut printer = SerialPrinter::new(&config.address)?;
            printer.write(&test_data)?;
            printer.flush()?;
            printer.close()?;
        }
        ConnectionType::Network => {
            let mut printer = NetworkPrinter::new(&config.address)?;
            printer.write(&test_data)?;
            printer.flush()?;
            printer.close()?;
        }
    }

    Ok(())
}
