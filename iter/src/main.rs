// iterator 트레잇 참고: https://doc.rust-lang.org/std/iter/index.html#iterator
fn main() {
    fn iterator_demonstration() {
        let v1 = vec![3, 4, 5];

        // iter: 불변 값들의 iter를 반환
        // into_iter: 소유권을 갖고 소유된 값들을 iter를 반환
        // iter_mut: 가변 참조에 대한 반환
        let mut v1_iter = v1.iter();

        // iter 할때마다 소모함.
        println!("{:?} {:?}", v1_iter.next(), Some(&3));

        // assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&4));
        assert_eq!(v1_iter.next(), Some(&5));
        assert_eq!(v1_iter.next(), None);

        let v1_iter_2 = v1.iter();

        // sum이 소유권을 가져가기 때문에 추후 v1_iter_2는 사용 불가능!
        let total: i32 = v1_iter_2.sum();

        println!("sum: {:?}", total);


        let v2: Vec<i32> = vec![1, 2, 3];
        // collect
        let v3: Vec<_> = v2.iter().map(|x| x * 2).collect();

        // Map struct를 반환할 뿐 Vec를 반환하지 않음. collect()를 해주어야 적용된 Vec이 반환
        let v2_map = v2.iter().map(|x| x + 3);
        // filter은 map에 비해서 & 하나가 더 붙는데 이유가 뭐임???
        let v2_filter = v2.iter().filter(|x| x >= &&2);

        println!("{:?}", v2_map);
        println!("{:?}", v2_filter);

        let v2_map_collected: Vec<_> = v2_map.collect();
        let v2_filter_collected: Vec<_> = v2_filter.collect();

        // println!("{:?}", v2_map); // ERR! v2_map은 collect()를 하고나서 소유권을 빼앗김

        assert_eq!(v2_map_collected, vec![4, 5, 6]);
        println!("v2_filter_collected: {:?}", v2_filter_collected);
        assert_eq!(v2_filter_collected, vec![&2, &3]);
        assert_eq!(v3, vec![2, 4, 6]);
    }

    iterator_demonstration();
}