use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused)]

/// Dijkstra's algorithm using adjacency matrix representation
fn dijkstra(graph: &Vec<Vec<i32>>, from: usize, to: usize) -> i32 {
    let n = graph.len();
    //初始化距离数组，起点的距离设为 0
    let mut dist = vec![i32::MAX; n];
    dist[from] = 0;
    // 标记节点是否已访问
    let mut visited = vec![false; n];

    // 遍历所有节点
    for _ in 0..n {
        // 找到当前未访问节点中距离最小的节点
        let mut min_dist = i32::MAX;
        let mut min_idx = None;

        for i in 0..n {
            if !visited[i] && dist[i] < min_dist {
                min_dist = dist[i];
                min_idx = Some(i);
            }
        }

        // 如果找不到未访问的节点，提前退出
        let min_idx = match min_idx {
            Some(node) => node,
            None => break,
        };

        // 标记该节点为已访问
        visited[min_idx] = true;

        // 更新邻居节点的距离
        for v in 0..n {
            let weight = graph[min_idx][v];
            if weight > 0 && !visited[v] {
                // 只处理有边相连且未访问的节点
                let new_dist = dist[min_idx] + weight;
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                }
            }
        }
    }

    dist[to]
}

#[derive(Debug)]
struct Edge {
    to: usize,
    cost: i32,
}

// 图结构
// key  value
// 1    [2, 3]
// 2    [3, 5]
fn dijkstra_heap(graph: &HashMap<usize, Vec<Edge>>, start: usize) -> HashMap<usize, i32> {
    let mut dists = HashMap::new();
    let mut heap = BinaryHeap::new();

    dists.insert(start, 0);
    heap.push((Reverse(0), start));

    while let Some((Reverse(cost), node)) = heap.pop() {
        // 如果当前节点的距离已经大于记录的最短距离，则跳过
        if cost > *dists.get(&node).unwrap_or(&i32::MAX) {
            continue;
        }

        // 遍历当前节点的邻居
        if let Some(edges) = graph.get(&node) {
            for edge in edges {
                let next_node = edge.to;
                let new_cost = cost + edge.cost;

                // 如果找到更短的路径，则更新距离并加入堆中
                if new_cost < *dists.get(&next_node).unwrap_or(&i32::MAX) {
                    dists.insert(next_node, new_cost);
                    heap.push((Reverse(new_cost), next_node));
                }
            }
        }
    }

    dists
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let graph = vec![
            vec![0, 4, 1, 0, 0],
            vec![4, 0, 2, 1, 0],
            vec![1, 2, 0, 5, 0],
            vec![0, 1, 5, 0, 3],
            vec![0, 0, 0, 3, 0],
        ];
        let from = 0;
        let to = 4;
        let dist = dijkstra(&graph, from, to);
        assert_eq!(dist, 7);
    }

    #[test]
    fn test_heap() {
        // 构建图的邻接表表示
        let mut graph = HashMap::new();
        graph.insert(0, vec![Edge { to: 1, cost: 4 }, Edge { to: 2, cost: 1 }]);
        graph.insert(1, vec![Edge { to: 3, cost: 1 }]);
        graph.insert(2, vec![Edge { to: 1, cost: 2 }, Edge { to: 3, cost: 5 }]);
        graph.insert(3, vec![]);

        // 起点
        let start = 0;

        // 计算最短路径
        let shortest_distances = dijkstra_heap(&graph, start);

        // 输出结果
        println!("从节点 {} 出发的最短路径：", start);
        for (node, distance) in &shortest_distances {
            println!("到节点 {}: 距离 {}", node, distance);
        }
    }
}
