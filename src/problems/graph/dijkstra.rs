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
}
