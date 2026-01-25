# Probook

Probook is a desktop invoicing and business management application for freelancers, self-employed professionals, and small businesses.

Built with **Tauri** (Rust) + **React** + **TypeScript**.

![Probook](icon.png)

## Features

- **Client Management** - Store and organize customer information
- **Product Catalog** - Manage products and services with pricing
- **Quotes** - Create, send, and track quotes with conversion to invoices
- **Invoices** - Generate professional invoices with automatic numbering
- **Delivery Notes** - Track deliveries linked to quotes/invoices
- **Payments** - Record and track payment history
- **PDF Generation** - Export documents as PDFs with company branding
- **Reports** - Revenue analytics, outstanding payments, quote conversion rates
- **Backup & Restore** - Local automatic backups + encrypted manual exports
- **Multi-language** - English, French, and Arabic (RTL support)
- **Dark Mode** - System-aware theming

## Tech Stack

### Frontend
- React 18 + TypeScript
- Vite (build tool)
- Tailwind CSS (styling)
- TanStack Query (server state)
- Zustand (UI state)
- React Hook Form + Zod (forms & validation)
- @react-pdf/renderer (PDF generation)
- i18next (internationalization)

### Backend
- Tauri (Rust-based desktop framework)
- SQLite via sqlx (database)
- Argon2 (password hashing)
- AES-256-GCM (backup encryption)

### Platforms
- Windows
- macOS