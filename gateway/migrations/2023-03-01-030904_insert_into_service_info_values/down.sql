-- This file should undo anything in `up.sql`
DELETE FROM
    "service_info"
WHERE
    id IN (
        select
            id
        from
            "service_info"
    );