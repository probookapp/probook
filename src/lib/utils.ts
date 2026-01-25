import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import i18n from "@/i18n";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

function getLocale(): string {
  const lang = i18n.language || "fr";
  // Map language codes to locale codes
  // Use -u-nu-latn for Arabic to keep Western/Latin numerals
  const localeMap: Record<string, string> = {
    fr: "fr-FR",
    en: "en-US",
    ar: "ar-SA-u-nu-latn",
  };
  return localeMap[lang] || "fr-FR";
}

export function formatCurrency(amount: number): string {
  return new Intl.NumberFormat(getLocale(), {
    style: "currency",
    currency: "EUR",
  }).format(amount);
}

export function formatDate(date: string): string {
  const formatted = new Intl.DateTimeFormat(getLocale(), {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
  }).format(new Date(date));
  // Remove RTL/LTR marks that Arabic locale adds
  return formatted.replace(/[\u200E\u200F\u202A-\u202E]/g, "");
}

export function formatDateISO(date: Date): string {
  return date.toISOString().split("T")[0];
}

export function calculateLineTotal(
  quantity: number,
  unitPrice: number,
  vatRate: number
): { totalHt: number; totalVat: number; totalTtc: number } {
  const totalHt = quantity * unitPrice;
  const totalVat = totalHt * (vatRate / 100);
  const totalTtc = totalHt + totalVat;
  return { totalHt, totalVat, totalTtc };
}

export function generateQuoteNumber(prefix: string, nextNumber: number): string {
  const year = new Date().getFullYear();
  const paddedNumber = String(nextNumber).padStart(4, "0");
  return `${prefix}${year}-${paddedNumber}`;
}

export function generateInvoiceNumber(prefix: string, nextNumber: number): string {
  const year = new Date().getFullYear();
  const paddedNumber = String(nextNumber).padStart(4, "0");
  return `${prefix}${year}-${paddedNumber}`;
}

// French number to words conversion
const UNITS = ['', 'un', 'deux', 'trois', 'quatre', 'cinq', 'six', 'sept', 'huit', 'neuf', 'dix', 'onze', 'douze', 'treize', 'quatorze', 'quinze', 'seize', 'dix-sept', 'dix-huit', 'dix-neuf'];
const TENS = ['', 'dix', 'vingt', 'trente', 'quarante', 'cinquante', 'soixante', 'soixante', 'quatre-vingt', 'quatre-vingt'];

function convertHundreds(n: number): string {
  if (n === 0) return '';

  let result = '';
  const hundreds = Math.floor(n / 100);
  const remainder = n % 100;

  if (hundreds > 0) {
    if (hundreds === 1) {
      result = 'cent';
    } else {
      result = UNITS[hundreds] + ' cent';
    }
    if (remainder === 0 && hundreds > 1) {
      result += 's';
    }
    if (remainder > 0) {
      result += ' ';
    }
  }

  if (remainder > 0) {
    if (remainder < 20) {
      result += UNITS[remainder];
    } else {
      const tensDigit = Math.floor(remainder / 10);
      const unitDigit = remainder % 10;

      if (tensDigit === 7 || tensDigit === 9) {
        // 70-79 uses soixante-dix, 90-99 uses quatre-vingt-dix
        const base = tensDigit === 7 ? 6 : 8;
        const added = tensDigit === 7 ? 10 + unitDigit : 10 + unitDigit;
        result += TENS[base];
        if (added === 11 && tensDigit === 7) {
          result += ' et onze';
        } else {
          result += '-' + UNITS[added];
        }
      } else if (tensDigit === 8) {
        result += TENS[8];
        if (unitDigit === 0) {
          result += 's';
        } else {
          result += '-' + UNITS[unitDigit];
        }
      } else {
        result += TENS[tensDigit];
        if (unitDigit === 1 && tensDigit !== 8) {
          result += ' et un';
        } else if (unitDigit > 0) {
          result += '-' + UNITS[unitDigit];
        }
      }
    }
  }

  return result;
}

function convertThousands(n: number): string {
  if (n === 0) return 'zéro';
  if (n < 0) return 'moins ' + convertThousands(-n);

  let result = '';

  // Millions
  const millions = Math.floor(n / 1000000);
  if (millions > 0) {
    if (millions === 1) {
      result += 'un million';
    } else {
      result += convertHundreds(millions) + ' millions';
    }
    n %= 1000000;
    if (n > 0) result += ' ';
  }

  // Thousands
  const thousands = Math.floor(n / 1000);
  if (thousands > 0) {
    if (thousands === 1) {
      result += 'mille';
    } else {
      result += convertHundreds(thousands) + ' mille';
    }
    n %= 1000;
    if (n > 0) result += ' ';
  }

  // Hundreds
  if (n > 0) {
    result += convertHundreds(n);
  }

  return result;
}

export function numberToFrenchWords(amount: number): string {
  const euros = Math.floor(amount);
  const cents = Math.round((amount - euros) * 100);

  let result = '';

  if (euros === 0) {
    result = 'zéro euro';
  } else if (euros === 1) {
    result = 'un euro';
  } else {
    result = convertThousands(euros) + ' euros';
  }

  if (cents > 0) {
    result += ' et ';
    if (cents === 1) {
      result += 'un centime';
    } else {
      result += convertThousands(cents) + ' centimes';
    }
  }

  // Capitalize first letter
  return result.charAt(0).toUpperCase() + result.slice(1);
}
