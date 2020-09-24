pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    if let Some(mut head) = head.as_mut() {
        fn split_node(mut node: Box<ListNode>, nodes: &mut Vec<Option<Box<ListNode>>>) {
            if let Some(next) = node.next.take() {
                split_node(next, nodes);
            }
            nodes.push(Some(node));
        }

        let mut nodes = vec![];
        if let Some(next) = head.next.take() {
            split_node(next, &mut nodes);
        }
        // println!("{:?}", nodes);

        let mut head_index = 0;
        let mut tail_index = nodes.len();
        let mut pick_from_head = true;
        println!("{}, {}", head_index, tail_index);
        while tail_index >= head_index + 1 {
            let node = if pick_from_head {
                head_index += 1;
                nodes[head_index-1].take()
            } else {
                tail_index -= 1;
                nodes[tail_index].take()
            };
            pick_from_head = !pick_from_head;

            head.next = node;
            head = head.next.as_mut().unwrap();
        }
    }
}