#[derive(Debug, Clone)]
pub struct FsAps<T>  {
    fs: Vec<T>,
    aps: Vec<usize>
}

impl FsAps<T> {
    pub fn new() -> Self {
        return Self {
            fs: Vec::new(),
            aps: Vec::new()
        }
    }

    pub fn from_vectors(from_fs: Vec<T>, from_aps: Vec<usize>) -> Self {
        return Self {
            fs: from_fs,
            aps: from_aps
        };
    }

    pub fn add_element<T>(element: T, parents: Option<&[T]>, childs: Option<&[T]>) -> Self {
        
    }

    pub fn remove_element() -> Self {
        unimplemented!();
    }

    pub fn get_successors(Self, index: usize) -> Result<Option<&[T]>, &'static str> {
        if index >= Self.aps.len() {
            return Err("The index overflow APS size");
        }

        let start_range_index: usize = Self.aps.get(index).unwrap().clone();
        let end_range_index: usize = Self.aps.get(index + 1).unwrap().clone();

        let successors = Self.fs.iter()
            .enumerate()
            .filter(|(index, _)| index >= &start_range_index && index < &end_range_index)
            .map(|(_, element)| element.clone())
            .collect::Vec<T>();

        if successors.is_empty() {
            return Ok(None);
        }

        return Ok(Some(successors));
    }

    pub fn get_descendants() {
        unimplemented!();
    }

    pub fn get_predecessors() {
        unimplemented!();
    }

    pub fn get_ancestors() {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn name() {
        unimplemented!();
    }
}