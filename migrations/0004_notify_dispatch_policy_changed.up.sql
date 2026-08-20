CREATE FUNCTION notify_dispatch_policy_changed()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify(
        'dispatch_policy_changed',
        NEW.id::text
    );

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER dispatch_policy_changed_trigger
AFTER INSERT ON dispatch_policy
FOR EACH ROW
EXECUTE FUNCTION notify_dispatch_policy_changed();
