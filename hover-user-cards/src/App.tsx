import React from "react";
import { StreamerSocialMediaCard, User } from "./socialMediaCard";
import {faTwitch, faYoutube} from "@fortawesome/free-brands-svg-icons"

function App() {
  const users = [
    {
      name: "Ottomated",
      short_about: "He do be developing",
      image:
        "https://pbs.twimg.com/profile_images/1161038324561113088/lQwcqFw6_400x400.png",
      twitter: "https://twitter.com/Ottomated_",
      stream: {
        link: "https://www.twitch.tv/ottomated",
        icon: faTwitch,
      },
    } as User,
    {
      name: "Ludwig Ahgren",
      short_about: "Otto's employer",
      image:
        "https://pbs.twimg.com/profile_images/1388667618932920325/-tjCvvbj_400x400.jpg",
      twitter: "https://twitter.com/LudwigAhgren",
      stream: {
        link: "https://www.youtube.com/c/Ludwigahgren/live",
        icon: faYoutube,
      },
    } as User,
    {
      name: "Illumina1337",
      short_about: "Mr. Speedrunner",
      image:
        "https://pbs.twimg.com/profile_images/1401452370786017282/EEdHBAHM_400x400.png",
      twitter: "https://twitter.com/IlluminaHD",
      stream: {
        link: "https://www.twitch.tv/illumina1337",
        icon: faTwitch,
      },
    } as User,
  ];

  const cards = users.map((user) => <StreamerSocialMediaCard user={user} />);

  return (
    <div className="App">
      <div className="cards">{cards}</div>
    </div>
  );
}

export default App;
