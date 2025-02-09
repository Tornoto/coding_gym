#![allow(unused)]

#[derive(Debug)]
struct Edge {
    source: usize,
    target: usize,
    weight: i32,
}
/// 图形式为: 节点1，节点2，边权重
/// n: 图中节点数
/// source: 起始节点
fn bellman_ford(edges: &Vec<Edge>, n: usize, source: usize) -> Option<Vec<i32>> {
    // 初始化距离矩阵
    let mut dist = vec![i32::MAX; n];
    dist[source] = 0;

    // 执行n-1轮
    // 为什么n-1轮？
    // 1. 在最短路径中，从源节点到任意节点的路径最多包含 n−1 条边。
    // 2. 在最坏情况下，最短路径的信息需要经过 n−1 次松弛操作才能传播到所有节点。节点按链式排列。
    for _ in 0..n {
        for edge in edges {
            let (u, v, w) = (edge.source, edge.target, edge.weight);
            if dist[u] != i32::MAX && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }

    for edge in edges {
        let (u, v, w) = (edge.source, edge.target, edge.weight);
        // 如果仍然可以松弛，则说明存在负权环
        if dist[u] != i32::MAX && dist[u] + w < dist[v] {
            return None;
        }
    }
    Some(dist)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let n = 5;
        let edges = vec![
            Edge {
                source: 0,
                target: 1,
                weight: 4,
            },
            Edge {
                source: 0,
                target: 2,
                weight: 1,
            },
            Edge {
                source: 1,
                target: 3,
                weight: 1,
            },
            Edge {
                source: 2,
                target: 1,
                weight: 2,
            },
            Edge {
                source: 2,
                target: 3,
                weight: 5,
            },
            Edge {
                source: 3,
                target: 4,
                weight: 3,
            },
        ];

        let source = 0;
        if let Some(dist) = bellman_ford(&edges, n, source) {
            println!("从节点 {} 到其他节点的最短距离：", source);
            for i in 0..n {
                println!("到节点 {} 的距离: {}", i, dist[i]);
            }
        }
    }
}
