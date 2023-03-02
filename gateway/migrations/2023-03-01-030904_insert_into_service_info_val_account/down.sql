-- This file should undo anything in `up.sql`
DELETE FROM
    "service_info"
WHERE
    "name" = 'account';