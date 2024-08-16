use header::EntityTag;

header! {
    #[doc="`If-None-Match` header, defined in"]
    #[doc="[RFC7232](https://tools.ietf.org/html/rfc7232#section-3.2)"]
    #[doc=""]
    #[doc="The `If-None-Match` header field makes the request method conditional"]
    #[doc="on a recipient cache or origin server either not having any current"]
    #[doc="representation of the target resource, when the field-value is \"*\","]
    #[doc="or having a selected representation with an entity-tag that does not"]
    #[doc="match any of those listed in the field-value."]
    #[doc=""]
    #[doc="A recipient MUST use the weak comparison function when comparing"]
    #[doc="entity-tags for If-None-Match (Section 2.3.2), since weak entity-tags"]
    #[doc="can be used for cache validation even if there have been changes to"]
    #[doc="the representation data."]
    #[doc=""]
    #[doc="# ABNF"]
    #[doc="```plain"]
    #[doc="If-None-Match = \"*\" / 1#entity-tag"]
    #[doc="```"]
    #[doc=""]
    #[doc="# Example values"]
    #[doc="* `\"xyzzy\"`"]
    #[doc="* `W/\"xyzzy\"`"]
    #[doc="* `\"xyzzy\", \"r2d2xxxx\", \"c3piozzzz\"`"]
    #[doc="* `W/\"xyzzy\", W/\"r2d2xxxx\", W/\"c3piozzzz\"`"]
    #[doc="* `*`"]
    (IfNoneMatch, "If-None-Match") => {Any / (EntityTag)+}

    test_if_none_match {
        test_header!(test1, vec![b"\"xyzzy\""]);
        test_header!(test2, vec![b"W/\"xyzzy\""]);
        test_header!(test3, vec![b"\"xyzzy\", \"r2d2xxxx\", \"c3piozzzz\""]);
        test_header!(test4, vec![b"W/\"xyzzy\", W/\"r2d2xxxx\", W/\"c3piozzzz\""]);
        test_header!(test5, vec![b"*"]);
    }
}

#[cfg(test)]
mod tests {
    use super::IfNoneMatch;
    use header::Header;
    use header::EntityTag;

    #[test]
    fn test_if_none_match() {
        let mut if_none_match: Option<IfNoneMatch>;

        if_none_match = Header::parse_header([b"*".to_vec()].as_ref());
        assert_eq!(if_none_match, Some(IfNoneMatch::Any));

        if_none_match = Header::parse_header([b"\"foobar\", W/\"weak-etag\"".to_vec()].as_ref());
        let mut entities: Vec<EntityTag> = Vec::new();
        let foobar_etag = EntityTag::new(false, "foobar".to_string());
        let weak_etag = EntityTag::new(true, "weak-etag".to_string());
        entities.push(foobar_etag);
        entities.push(weak_etag);
        assert_eq!(if_none_match, Some(IfNoneMatch::Items(entities)));
    }
}

bench_header!(bench, IfNoneMatch, { vec![b"W/\"nonemptytag\"".to_vec()] });
