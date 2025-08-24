pub struct Compare<'s, 'd, T, U> {
    to_remove: Vec<&'d U>,
    to_add: Vec<&'s T>,
}

impl<'s, 'd, T: PartialEq<U>, U> Compare<'s, 'd, T, U> {
    pub fn new(
        src: impl IntoIterator<Item = &'s T>,
        dest: impl IntoIterator<Item = &'d U>,
    ) -> Self {
        self::compare(src.into_iter(), dest.into_iter())
    }

    pub fn to_remove(&self) -> &[&'d U] {
        &self.to_remove
    }

    pub fn to_add(&self) -> &[&'s T] {
        &self.to_add
    }
}

fn compare<'s, 'd, T: PartialEq<U>, U>(
    src: impl IntoIterator<Item = &'s T>,
    dest: impl IntoIterator<Item = &'d U>,
) -> Compare<'s, 'd, T, U> {
    let mut src = src.into_iter();
    let mut dest = dest.into_iter();

    let mut to_remove = vec![];
    let mut to_add = vec![];

    let mut src_next = src.next();
    let mut dest_next = dest.next();

    loop {
        match (src_next, dest_next) {
            (Some(src_item), Some(dest_item)) => {
                if src_item == dest_item {
                    src_next = src.next();
                    dest_next = dest.next();
                } else {
                    to_remove.push(dest_item);
                    dest_next = dest.next();
                }
            }

            (Some(src_item), None) => {
                to_add.push(src_item);
                to_add.extend(src);
                break;
            }

            (None, Some(dest_item)) => {
                to_remove.push(dest_item);
                to_remove.extend(dest);
                break;
            }

            (None, None) => break,
        }
    }

    Compare { to_remove, to_add }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_empty() {
        let src = Vec::<i32>::new();
        let dest = Vec::<i32>::new();

        let result = compare(&src, &dest);

        assert_eq!(result.to_add(), &Vec::<&i32>::new());
        assert_eq!(result.to_remove(), &Vec::<&i32>::new());
    }

    #[test]
    fn test_src_empty() {
        let src = Vec::<i32>::new();
        let dest = vec![1, 2, 3];

        let result = compare(&src, &dest);

        assert_eq!(result.to_add(), &Vec::<&i32>::new());
        assert_eq!(result.to_remove(), &[&1, &2, &3]);
    }

    #[test]
    fn test_dest_empty() {
        let src = vec![1, 2, 3];
        let dest = Vec::<i32>::new();

        let result = compare(&src, &dest);

        assert_eq!(result.to_add(), &[&1, &2, &3]);
        assert_eq!(result.to_remove(), &Vec::<&i32>::new());
    }

    #[test]
    fn test_equal() {
        let src = vec![1, 2, 3];
        let dest = vec![1, 2, 3];

        let result = compare(&src, &dest);

        assert_eq!(result.to_add(), &Vec::<&i32>::new());
        assert_eq!(result.to_remove(), &Vec::<&i32>::new());
    }

    #[test]
    fn test_remove() {
        let src = vec![1, 3];
        let dest = vec![1, 2, 3];

        let result = compare(&src, &dest);

        assert_eq!(result.to_add(), &Vec::<&i32>::new());
        assert_eq!(result.to_remove(), &[&2]);
    }

    #[test]
    fn test_add() {
        let src = vec![1, 2, 3, 4];
        let dest = vec![1, 2, 3];

        let result = compare(&src, &dest);

        assert_eq!(result.to_add(), &[&4]);
        assert_eq!(result.to_remove(), &Vec::<&i32>::new());
    }

    #[test]
    fn test_longer_src() {
        let src = vec![2, 4, 5, 6];
        let dest = vec![1, 2, 3];

        let result = compare(&src, &dest);

        assert_eq!(result.to_add(), &[&4, &5, &6]);
        assert_eq!(result.to_remove(), &[&1, &3]);
    }

    #[test]
    fn test_longer_dest() {
        let src = vec![2, 4];
        let dest = vec![1, 2, 3];

        let result = compare(&src, &dest);

        assert_eq!(result.to_add(), &[&4]);
        assert_eq!(result.to_remove(), &[&1, &3]);
    }
}
