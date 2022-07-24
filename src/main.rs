
fn main() {
    println!("Hello, world!");
}

fn stop_at(route:&Vec<usize>, minute:usize) -> usize {
    let index = minute % route.len();
    return route[index]
}
fn minute_to_complete_gossip(routes:Vec<Vec<usize>>) -> Option<usize> {
    if routes.len() == 0 {
        return None
    }
    if routes.len() == 1 {
        return Some(0)
    }
    if stop_at(&routes[0],0) == stop_at(&routes[1],0) {
        return Some(0)
    }
    if stop_at(&routes[0],1) == stop_at(&routes[1],1) {
        return Some(1)
    }
    return None
}

mod tests {

    use super::*;

    #[test] 
    fn given_one_driver_only_result_is_minute_zero() {
        let routes = vec![vec![4807]];
        assert_eq!(Some(0), minute_to_complete_gossip(routes))
    }

    #[test]
    fn given_no_driver_result_is_never() {
        let routes = vec![];
       // let routes = network![]; // should not compile : network with not routes
        assert_eq!(None, minute_to_complete_gossip(routes))
    }

    #[test]
    fn given_no_common_stops_result_is_never() {
        let routes = vec![vec![42]
                         ,vec![17]];
      //  let routes = network![line![42]   // should not compile : line with less than 2 stops
      //                       ,line![17]];
        assert_eq!(None, minute_to_complete_gossip(routes))
    }
    #[test]
    fn given_two_commons_stop_at_minute_0_result_is_zero() {
        let routes = vec![vec![42]
                         ,vec![42]];
        assert_eq!(Some(0), minute_to_complete_gossip(routes))
    }
    #[test]
    fn given_two_commons_stop_at_minute_1_result_is_one() {
        let routes = vec![vec![17,42]
                         ,vec![23,42]];
      //  let routes = network![line![17,42]
      //                       ,line![23,42]];
        assert_eq!(Some(1), minute_to_complete_gossip(routes))
    }
}
   
