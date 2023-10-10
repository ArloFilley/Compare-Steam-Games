function request() {
	let steam1 = document.getElementById("user1").value;
	let steam2 = document.getElementById("user2").value;
    let filterTime = document.getElementById("filterTime").value;

    let data = {
        steamids: [steam1, steam2],
        filtertime: Number(filterTime)
    }

    var http = new XMLHttpRequest();
    var url = '../';
    var body = JSON.stringify(data);
    http.open('POST', url, true);

    //Send the proper header information along with the request
    http.setRequestHeader('Content-type', 'application/json');

    http.onreadystatechange = function() {//Call a function when the state changes.
        if(http.readyState == 4 && http.status == 200) {
            data = JSON.parse(http.responseText);
		console.log(data);
		draw_chart(data);
        }
    }
    http.send(body);
}

function draw_chart(data) {
	let games = [];
	let playtimes1 = [];
	let playtimes2 = [];

	for (let i = 0; i < data.games.length; i++) {
		games.push(data.games[i].name);
		playtimes1.push(data.games[i].playtimes[0] / 60)
		playtimes2.push(data.games[i].playtimes[1] / 60)
	}

  const ctx = document.getElementById('playtime-chart');

	const data2 = {
	  labels: games,
	  datasets: [
	    {
	      label: data.users[0].username,
	      data: playtimes1,
	      backgroundColor: 'rgba(255, 99, 132, 0.8)',
	      stack: 'Stack 0',
	    },
	    {
	      label: data.users[1].username,
	      data: playtimes2,
	      backgroundColor: 'rgba(54, 162, 235, 0.8)',
	      stack: 'Stack 0',
	    },
	  ]
	};

	const options =  {
    plugins: {
      title: {
        display: true,
        text: 'Time Played in Hours'
      },
    },
    responsive: true,
    interaction: {
      intersect: false,
    },
    scales: {
      x: {
        stacked: true,
      },
      y: {
        stacked: true
      }
    }
  } 

  new Chart(ctx, 
	  {
    	type: 'bar',
    	data: data2,
    	options: options
 	  }
  );

}
