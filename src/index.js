const width = 640;
const height = 400;
const marginTop = 20;
const marginRight = 20;
const marginBottom = 30;
const marginLeft = 40;

async function getData() {
    const response = await fetch("/api/cpus");

    if (response.status != 200) {
        throw new Error("Something has gone wrong");
    }

    let data = await response.json();

    return data;
}

document.addEventListener("DOMContentLoaded", async () => {
    const svg = d3.select("#chart")
        .append("svg")
        .attr("width", width + marginLeft + marginRight)
        .attr("height", height + marginTop + marginBottom);

    const xScale = d3.scaleLinear()
        .domain([0, 100])  // Replace 'data' with your array
        .range([marginLeft, width - marginRight]);

    const yScale = d3.scaleLinear()
        .domain([0, 100])
        .range([height - marginBottom, marginTop]);

    const rects = svg.selectAll("rect");

    setInterval(async () => {
        const data = await getData();

        rects.data(data)
            .join(
                enter => enter.append("rect"),
                update => update,
                exit => exit.remove()
            )
            .attr("x", d => xScale(d))
            .attr("y", d => yScale(d))
            .attr("width", 100)
            .attr("height", d => height - yScale(d))
            .attr("fill", "steelblue");

    }, 1000);
});
