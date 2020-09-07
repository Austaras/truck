use crate::*;

impl<P> Vertex<P> {
    /// constructor
    /// # Examples
    /// ```
    /// use truck_topology::*;
    /// let v0 = Vertex::new(()); // a vertex whose geometry is the empty tuple.
    /// let v1 = Vertex::new(()); // another vertex
    /// let v2 = v0.clone(); // a cloned vertex
    /// assert_ne!(v0, v1);
    /// assert_eq!(v0, v2);
    /// ```
    #[inline(always)]
    pub fn new(point: P) -> Vertex<P> {
        Vertex { point: Arc::new(Mutex::new(point)) }
    }

    /// Creates `len` distinct vertices and return them by vector.
    /// # Examples
    /// ```
    /// use truck_topology::Vertex;
    /// let v = Vertex::news(&[(), (), ()]);
    /// assert_eq!(v.len(), 3);
    /// assert_ne!(v[0], v[2]);
    /// ```
    #[inline(always)]
    pub fn news(points: &[P]) -> Vec<Vertex<P>> where P: Copy {
        points.iter().map(|p| Vertex::new(*p)).collect()
    }

    /// Tries to lock the mutex of the contained point.
    /// # Examples
    /// ```
    /// use truck_topology::*;
    /// let v0 = Vertex::new(0);
    /// let v1 = v0.clone();
    /// 
    /// // Two vertices have the same content.
    /// assert_eq!(*v0.try_lock_point().unwrap(), 0);
    /// assert_eq!(*v1.try_lock_point().unwrap(), 0);
    /// 
    /// {
    ///     let mut point = v0.try_lock_point().unwrap();
    ///     *point = 1;
    /// }
    /// // The contents of two vertices are synchronized.
    /// assert_eq!(*v0.try_lock_point().unwrap(), 1);
    /// assert_eq!(*v1.try_lock_point().unwrap(), 1);
    /// 
    /// // Check the behavior of `try_lock`.
    /// let lock = v0.try_lock_point();
    /// assert!(v1.try_lock_point().is_err());    
    /// ```
    #[inline(always)]
    pub fn try_lock_point(&self) -> TryLockResult<MutexGuard<P>> { self.point.try_lock() }

}

impl<P> Clone for Vertex<P> {
    #[inline(always)]
    fn clone(&self) -> Vertex<P> {
        Vertex { point: Arc::clone(&self.point) }
    }
}

impl<P> PartialEq for Vertex<P> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(Arc::as_ptr(&self.point), Arc::as_ptr(&other.point))
    }
}

impl<P> Eq for Vertex<P> {}

impl<P> Hash for Vertex<P> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(Arc::as_ptr(&self.point), state);
    }
}
