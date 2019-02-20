import React from 'react';
import { Route, Switch } from 'react-router';
import Tabs, { TabItem } from '@polkadot/ui-app/Tabs';
import { withCalls } from '@polkadot/ui-api/with';
import { AppProps, I18nProps } from '@polkadot/ui-app/types';
import { AccountId, Hash } from '@polkadot/types';

// our app-specific styles
import './index.css';

// local imports and components
import translate from './translate';
import Dashboard from './Dashboard';
import Council from './Council';
import Applicants from './Applicants';
import Votes from './Votes';
import Reveals from './Reveals';
import { queryToProp } from '@polkadot/joy-utils/index';
import { Seat } from '@polkadot/joy-utils/types';

// define out internal types
type Props = AppProps & I18nProps & {
  activeCouncil?: Seat[],
  applicants?: AccountId[],
  commitments?: Hash[]
};

type State = {};

class App extends React.PureComponent<Props, State> {

  state: State = {};

  private buildTabs (): TabItem[] {
    const { t, activeCouncil = [], applicants = [], commitments = [] } = this.props;
    return [
      {
        name: 'election',
        text: t('Dashboard')
      },
      {
        name: 'council',
        text: `Council (${activeCouncil.length})`
      },
      {
        name: 'applicants',
        text: `Applicants (${applicants.length})`
      },
      {
        name: 'votes',
        text: `Votes (${commitments.length})`
      },
      {
        name: 'reveals',
        text: t('Reveal a vote')
      }
    ];
  }

  render () {
    const { basePath } = this.props;
    const tabs = this.buildTabs();
    return (
      <main className='election--App'>
        <header>
          <Tabs basePath={basePath} items={tabs} />
        </header>
        <Switch>
          <Route path={`${basePath}/council`} component={Council} />
          <Route path={`${basePath}/applicants`} component={Applicants} />
          <Route path={`${basePath}/votes`} component={Votes} />
          <Route path={`${basePath}/reveals`} component={Reveals} />
          <Route component={Dashboard} />
        </Switch>
      </main>
    );
  }
}

export default translate(
  withCalls<Props>(
    queryToProp('query.council.activeCouncil'),
    queryToProp('query.councilElection.applicants'),
    queryToProp('query.councilElection.commitments')
  )(App)
);
