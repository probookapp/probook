-- Reminders for payment and quote follow-ups

CREATE TABLE IF NOT EXISTS reminders (
    id TEXT PRIMARY KEY,
    reminder_type TEXT NOT NULL,
    document_type TEXT NOT NULL,
    document_id TEXT NOT NULL,
    scheduled_date DATE NOT NULL,
    sent_at TIMESTAMPTZ,
    message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
