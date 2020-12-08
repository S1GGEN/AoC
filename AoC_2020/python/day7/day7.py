import networkx as nx
# import matplotlib.pyplot as plt

def one(task_input):
    graph = make_graph(task_input)
    in_edges = get_in_edges(graph, "shiny_gold")
    return len(set(in_edges))

def get_in_edges(graph, node):
    results = []
    in_edges = graph.in_edges([node])
    for edge in in_edges:
        in_edge, _ = edge
        results.append(in_edge)
        results += get_in_edges(graph, in_edge)
    return results

def two(task_input):
    graph = make_graph(task_input)
    two_sum = get_out_sum(graph, "shiny_gold", 1)

    return two_sum

def get_out_sum(graph, node, multiplier):
    out_edge_sum = 0
    out_edges = graph.out_edges([node])
    for edge in out_edges:
        _, out_node = edge
        weight = int(graph.edges[node, out_node]['weight'])
        out_edge_sum += int(weight) * int(multiplier)
        out_edge_sum += get_out_sum(graph, out_node, weight * multiplier)
    # Store this value in the node, so that we can access it directly later:
    # graph.nodes[node]['sub_bags'] = x
    # https://networkx.org/documentation/stable/tutorial.html#node-attributes
    return out_edge_sum

def make_graph(task_input):
    graph = nx.DiGraph()

    for line in task_input:
        line_splitted = line.split(" ")
        this_bag = line_splitted[0] + "_" + line_splitted[1]

        if line_splitted[4] != "no":
            for i in range(4, len(line_splitted), 4):
                weight = line_splitted[i]
                sub_bag = line_splitted[i + 1] + "_" + line_splitted[i + 2]
                graph.add_edge(this_bag, sub_bag, weight=weight)


    # plt.tight_layout()
    # fig = plt.figure(figsize=(20, 20))
    # pos = nx.circular_layout(graph)
    # nx.draw_networkx(graph, pos=pos, arrows=True)
    # plt.savefig("graph.png", format="PNG")
    # plt.clf()

    return graph

def main():
    input = open('day7/input.txt', 'r')
    print("Day 7 part 1:", one(input))
    print("Day 7 part 2:", two(input))


main()
