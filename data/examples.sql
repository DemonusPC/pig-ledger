SELECT id as h_id, parent, name, (SELECT CASE WHEN child IS NOT NULL THEN (SELECT type from AccountsV2 WHERE id = child) ELSE NULL END) as acc_type, (SELECT CASE WHEN child IS NOT NULL THEN (SELECT name from AccountsV2 WHERE id = child) ELSE NULL END) as acc_name, (SELECT CASE WHEN child IS NOT NULL THEN (SELECT balance from AccountsV2 WHERE id = child) ELSE NULL END) as balance, (SELECT CASE WHEN child IS NOT NULL THEN (SELECT balance from AccountsV2 WHERE id = child) ELSE NULL END) as currency, leaf FROM ExpensestHierarchies ORDER BY name DESC;