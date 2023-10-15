let data = {
    steamids: [],
    filtertime: 60
}

function addUser() {

    const url = document.getElementById("user").value;
    const regex = /\/id\/([^/]+)/;
    const match = url.match(regex);

    if (match) {
        const userIdOrVanityUrl = match[1];
        console.log(userIdOrVanityUrl);
        steamId = userIdOrVanityUrl;
    } else {
        console.log("URL format does not match.");
        alert("error in url format");
        return
    }


    if (data.steamids.includes(steamId)) {
        return;
    }

    data.steamids.push(steamId);

    let cards = document.getElementById("users");
    let card = document.createElement('button');
    card.textContent = "Loading...";
    card.className = "user-card"
    card.id = steamId;

    cards.appendChild(card);

    console.log(cards, card);
    request();
}

function create_card(user) {
    let cards = document.getElementById("users");
    let card = document.getElementById(user.user_id);

    if (card === null) {
        card = cards.childNodes[cards.childElementCount - 1];
    }
    
    cards.removeChild(card);
    card = document.createElement('button');
    card.textContent = user.username;
    card.className = "user-card";
    card.style.backgroundImage = `url(${user.avatar})`;

    cards.appendChild(card);

    console.log(cards, card);
}

function request() {
    let http = new XMLHttpRequest();
    let url = '../';
    let body = JSON.stringify(data);
    http.open('POST', url, true);

    http.setRequestHeader('Content-type', 'application/json');

    http.onreadystatechange = () => {
        if ( http.readyState == 4 && http.status == 200 ) {
            response = JSON.parse(http.responseText);
		    console.log(response);
		    draw_chart(response);
            create_card(response.users[response.users.length - 1]);
        } else {
            // alert("Server Error: Contact a maintainer")
        }
    }

    http.send(body);
}

function draw_chart(data) {
    if (!document.getElementsByTagName('canvas').length == 0) {
        document.body.removeChild(document.getElementsByTagName('canvas')[0]);
    };

    const ctx = document.createElement('canvas');

	let games = [];
    let playtimes = []
    let backgroundColors = [
        'rgba(255, 99, 132, 0.8)', 
        'rgba(54, 162, 235, 0.8)', 
        'rgba(54, 0, 235, 0.8)', 
        'rgba(54, 235, 235, 0.8)',
        'rgba(54, 235, 24, 0.8)'
    ];
    let datasets = [];

	for (let i = 0; i < data.games.length; i++) {
		games.push(data.games[i].name);
	}

    for (let j = 0; j < data.users.length; j++) {
        let playtime = [];
        for (let i = 0; i < data.games.length; i++) {
            playtime.push(data.games[i].playtimes[j] / 60)
        }
        playtimes.push(playtime);
    }

    for (let j = 0; j < data.users.length; j++) {
        datasets.push(
            {
                label: data.users[j].username,
                data: playtimes[j],
                backgroundColor: backgroundColors[j],
                stack: 'Stack 0'
            }
        )
    }

	const data2 = {
	  labels: games,
	  datasets: datasets,
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

  document.body.appendChild(ctx);
}
