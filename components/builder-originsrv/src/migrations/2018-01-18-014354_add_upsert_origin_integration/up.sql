CREATE OR REPLACE FUNCTION upsert_origin_integration_v1 (
  in_origin text,
  in_integration text,
  in_name text,
  in_body text
) RETURNS SETOF origin_integrations AS $$
  BEGIN
    RETURN QUERY
      INSERT INTO origin_integrations(origin, integration, name, body)
      VALUES (in_origin, in_integration, in_name, in_body)
      ON CONFLICT(origin, integration, name)
      DO UPDATE SET body = in_body RETURNING *;
    RETURN;
  END
$$ LANGUAGE plpgsql VOLATILE;
