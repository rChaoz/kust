pub trait ScopeFunctions {
    /// Calls the specified function with this value as an argument and returns the result of the function.
    ///
    /// Use when you need repeated access to a value in an expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt::Write;
    /// use kust::ScopeFunctions;
    ///
    /// let input = "\
    /// 8,4,19,30,1,0
    /// 1,4,7,11,9,5";
    ///
    /// let mut output = String::new();
    ///
    /// for line in input.lines() {
    ///     writeln!(
    ///         output,
    ///         "Average and median: {:?}",
    ///         line
    ///             .split(',')
    ///             .map(|s| s.parse::<i32>().unwrap())
    ///             .collect::<Vec<_>>()
    ///             .using(|v| (v.iter().sum::<i32>() / v.len() as i32, v[v.len() / 2]))
    ///     ).unwrap();
    /// }
    ///
    /// assert_eq!(output, "\
    /// Average and median: (10, 30)
    /// Average and median: (6, 11)
    /// ")
    /// ```
    fn using<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    /// Calls the specified function with an immutable reference to `self` and returns `self`.
    ///
    /// Use `also` when you want to perform a side effect using a value before returning it.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt::Write;
    /// use kust::ScopeFunctions;
    ///
    /// let mut output = String::new();
    ///
    /// let nums = [1, 2, 3, 4];
    /// let sum: i32 = nums.iter().map(|n| (n * 2).also(|d| writeln!(output, "{n} * 2 = {d}").unwrap())).sum();
    /// writeln!(output, "The sum is {sum}").unwrap();
    ///
    /// assert_eq!(output, "\
    /// 1 * 2 = 2
    /// 2 * 2 = 4
    /// 3 * 2 = 6
    /// 4 * 2 = 8
    /// The sum is 20
    /// ")
    /// ```
    fn also<F, R>(self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&Self) -> R,
    {
        f(&self);
        self
    }

    /// Calls the specified function with a mutable reference to `self` and returns `self`.
    ///
    /// Use `apply` when you want to modify an object before returning it.
    ///
    /// # Examples
    ///
    /// ```
    /// use kust::ScopeFunctions;
    ///
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// impl Person {
    ///     // Doesn't allow setting all fields, such as age
    ///     fn create(name: &str) -> Self {
    ///         Self {
    ///             name: String::from(name),
    ///             age: 0,
    ///         }
    ///     }
    ///
    ///     fn summary(&self) -> String {
    ///         format!("Name: {}, age: {}", self.name, self.age)
    ///     }
    /// }
    ///
    /// let infos = [("Mike", 34), ("Linda", 25)];
    /// let people = infos.map(|(name, age)| Person::create(name).apply(|p| p.age = age).summary());
    ///
    /// assert_eq!(people, [String::from("Name: Mike, age: 34"), String::from("Name: Linda, age: 25")]);
    /// ```
    fn apply<F, R>(mut self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&mut Self) -> R,
    {
        f(&mut self);
        self
    }
}

impl<T> ScopeFunctions for T {}
