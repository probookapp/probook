-- Normalize unit values from French strings to language-neutral keys.

-- Products table
UPDATE products SET unit = 'unit'      WHERE unit = 'unité';
UPDATE products SET unit = 'hour'      WHERE unit = 'heure';
UPDATE products SET unit = 'day'       WHERE unit = 'jour';
UPDATE products SET unit = 'month'     WHERE unit = 'mois';
UPDATE products SET unit = 'flat_rate' WHERE unit = 'forfait';
UPDATE products SET unit = 'piece'     WHERE unit = 'pièce';
UPDATE products SET unit = 'pallet'    WHERE unit = 'palette';
UPDATE products SET unit = 'sqm'       WHERE unit = 'm²';
UPDATE products SET unit = 'cbm'       WHERE unit = 'm³';

-- Also update the column default
ALTER TABLE products ALTER COLUMN unit SET DEFAULT 'unit';

-- Delivery note lines
UPDATE delivery_note_lines SET unit = 'unit'      WHERE unit = 'unité';
UPDATE delivery_note_lines SET unit = 'hour'      WHERE unit = 'heure';
UPDATE delivery_note_lines SET unit = 'day'       WHERE unit = 'jour';
UPDATE delivery_note_lines SET unit = 'month'     WHERE unit = 'mois';
UPDATE delivery_note_lines SET unit = 'flat_rate' WHERE unit = 'forfait';
UPDATE delivery_note_lines SET unit = 'piece'     WHERE unit = 'pièce';
UPDATE delivery_note_lines SET unit = 'pallet'    WHERE unit = 'palette';
UPDATE delivery_note_lines SET unit = 'sqm'       WHERE unit = 'm²';
UPDATE delivery_note_lines SET unit = 'cbm'       WHERE unit = 'm³';
