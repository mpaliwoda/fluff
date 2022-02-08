import { faTwitter, IconDefinition } from "@fortawesome/free-brands-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export type User = {
  name: string;
  short_about: string;
  image: string;
  twitter: string;
  stream: {
    link: string;
    icon: IconDefinition;
  };
};

export function StreamerSocialMediaCard(props: { user: User }): JSX.Element {
  const user = props.user;

  return (
    <div className="card">
      <div className="card__img">
        <img src={user.image} alt="" />
      </div>

      <div className="card__body">
        <span className="card__name">{user.name}</span>
        <span className="card__about">{user.short_about}</span>
      </div>

      <div className="card__soc card-soc">
        <div className="card-soc__col">
          <a href={user.twitter} className="card-soc__item">
            <FontAwesomeIcon icon={faTwitter} />
          </a>
        </div>
        <div className="card-soc__col">
          <a href={user.stream.link} className="card-soc__item">
            <FontAwesomeIcon icon={user.stream.icon} />
          </a>
        </div>
      </div>
    </div>
  );
}
