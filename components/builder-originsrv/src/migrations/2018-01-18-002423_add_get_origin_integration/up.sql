CREATE OR REPLACE FUNCTION get_origin_integration_v1 (
  in_origin text,
  in_integration text,
  in_name text
) RETURNS SETOF origin_integrations AS $$
  SELECT *
    FROM origin_integrations
   WHERE origin = in_origin
     AND integration = in_integration
     AND name = in_name
$$ LANGUAGE SQL STABLE;
