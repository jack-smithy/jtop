const apiUrl = '/api/cpus';

// Chart.js configuration
const chartConfig = {
    type: 'bar',
    data: {
        labels: [], // Will be populated with labels from API
        datasets: [{
            label: 'CPU usage',
            data: [], // Will be populated with data from API
            backgroundColor: 'rgba(54, 162, 235, 0.2)',
            borderColor: 'rgba(54, 162, 235, 1)',
            borderWidth: 1
        }]
    },
    options: {
        scales: {
            y: {
                beginAtZero: true,
                max: 100, // Set the maximum value to 100% for percentage data
                ticks: {
                    callback: value => value + '%' // Display percentage values
                }
            }
        }
    }
};

// Create the chart
const barChart = new Chart(document.getElementById('barChart'), chartConfig);

// Function to fetch data from API and update the chart
async function fetchDataAndUpdateChart() {
    try {
        const response = await fetch(apiUrl);
        const data = await response.json();

        // Update chart data
        chartConfig.data.labels = data.labels;
        chartConfig.data.datasets[0].data = data.values;

        // Update the chart
        barChart.update();
    } catch (error) {
        console.error('Error fetching data:', error);
    }
}

// Call the fetchDataAndUpdateChart function every second
setInterval(fetchDataAndUpdateChart, 1000);