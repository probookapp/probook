# Probook - Project Context

## Project Overview
Probook - Desktop invoicing and business management software for freelancers, self-employed professionals, and small businesses.

## Technology Stack

### Desktop Framework
- **Tauri** - Rust-based desktop container (lightweight, secure)

### Frontend
- **React 18+** with TypeScript
- **Vite** - Build tool
- **Tailwind CSS** - Styling
- **TanStack Query** - Server state management (caching Tauri command results)
- **Zustand** - UI state management
- **React Hook Form** - Form handling
- **Zod** - Validation schemas
- **@react-pdf/renderer** - PDF generation

### Backend (Rust via Tauri)
- **sqlx** - Async SQLite database access
- **argon2** - Password hashing
- **aes-gcm** - AES-256 encryption
- **serde** - Serialization
- **uuid** - ID generation
- **chrono** - Date/time handling

### Database
- **SQLite** - Local embedded database

### Platforms
- Windows
- macOS

## Architecture

```
[ React UI (TypeScript) ]
         |
   Tauri invoke() commands
         |
[ Rust Backend (Tauri Commands) ]
         |
   SQLite (via sqlx)
         |
[ Local File Storage ]
```

## Project Structure

```
probook/
├── src/                    # React frontend
│   ├── components/         # Reusable UI components
│   ├── features/           # Feature modules
│   │   ├── clients/
│   │   ├── products/
│   │   ├── quotes/
│   │   ├── invoices/
│   │   └── settings/
│   ├── hooks/              # Custom React hooks
│   ├── lib/                # Utilities, Tauri bindings
│   ├── stores/             # Zustand stores
│   └── types/              # TypeScript types
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri command handlers
│   │   ├── db/             # Database operations
│   │   ├── models/         # Data models
│   │   ├── services/       # Business logic
│   │   └── main.rs
│   ├── migrations/         # SQLite migrations
│   └── Cargo.toml
├── public/                 # Static assets
└── package.json
```

## Database Schema

### Core Tables
- **clients** - Customer/prospect information
- **products** - Products and services catalog
- **quotes** - Quote documents
- **quote_lines** - Line items for quotes
- **invoices** - Invoice documents
- **invoice_lines** - Line items for invoices
- **payments** - Payment records
- **company_settings** - User's business information

### Enums
- QuoteStatus: DRAFT, SENT, ACCEPTED, EXPIRED
- InvoiceStatus: DRAFT, ISSUED, PAID

## MVP Features (Phase 1)

### Included
- [ ] Client management (CRUD)
- [ ] Product/service management
- [ ] Quote creation and editing
- [ ] Quote → Invoice conversion
- [ ] Invoice management
- [ ] Payment tracking
- [ ] PDF generation
- [ ] OS printing
- [ ] Company branding (logo, info)
- [ ] Manual backup & restore
- [ ] Basic security (hashing, encryption)
- [ ] Invoice integrity hash

### Excluded from MVP
- Advanced legal compliance
- Electronic invoicing
- Automated reminders
- Multi-user support
- Cloud synchronization
- Advanced reporting

## Key Commands

```bash
# Development
npm run dev              # Start Vite dev server
npm run tauri dev        # Start Tauri in development mode

# Building
npm run build            # Build frontend
npm run tauri build      # Build distributable app

# Database
# Migrations run automatically on app start via Rust
```

## Type Safety Strategy

1. Define Rust structs with `serde::Serialize/Deserialize`
2. Use `specta` + `tauri-specta` to auto-generate TypeScript types
3. TanStack Query wraps Tauri invoke calls with proper typing

## Security Notes

- Passwords hashed with Argon2
- Sensitive data encrypted with AES-256-GCM
- Invoice signatures: SHA-256 hash of immutable fields
- Backups encrypted before export
- No plain-text secrets in database

## PDF Templates

Using @react-pdf/renderer with customizable components:
- Company header with logo
- Client information block
- Line items table
- Totals section
- Footer with legal mentions

## Conventions

- Use French accounting terms internally (HT = before tax, TTC = with tax, TVA = VAT)
- Currency: EUR (expandable later)
- Date format: ISO 8601 (YYYY-MM-DD) in DB, localized in UI
- IDs: UUID v4
