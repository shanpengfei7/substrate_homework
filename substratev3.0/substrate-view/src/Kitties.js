import React, { useEffect, useState } from 'react';
import { Form, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

import KittyCards from './KittyCards';

export default function Kitties (props) {
  const { api, keyring } = useSubstrate();
  const { accountPair } = props;

  const [count, setCount] = useState(0);
  const [dnas, setDnas] = useState([]);
  const [owners, setOwners] = useState([]);
  const [market, setMarket] = useState([]);

  const [kitties, setKitties] = useState([]);
  const [status, setStatus] = useState('');

  const fetchKitties = () => {
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态

    let unsubDnas = null;
    let unsubOwners = null;
    let unsubMarket = null;

    const asyncFetch = async () => {
      let kittiesCount = await api.query.kittiesModule.kittiesCount();
      let count = 0;
      if (!kittiesCount.isNone) {
        count = kittiesCount.unwrap().toNumber();
      }
      setCount(count);
    
      let indices = [];
      for(let i = 1; i <= count; i++) {
        indices.push(i);
      }

      unsubDnas = await api.query.kittiesModule.kitties.multi(
        indices,
        dnas => setDnas(dnas.map(dna => dna.value.toU8a()))
      );

      unsubOwners = await api.query.kittiesModule.owner.multi(
        indices,
        owners => setOwners(owners.map(owner => owner.toHuman()))
      );

      unsubMarket = await api.query.kittiesModule.kittiesMarket.multi(
        indices,
        market => setMarket(market.map(market => market.isSome && market.toHuman()))
      );
    };

    asyncFetch();

    // return the unsubscription cleanup function
    return () => {
      unsubDnas && unsubDnas();
      unsubOwners && unsubOwners();
      unsubMarket && unsubMarket();
    };
  }

  const populateKitties = () => {
    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    // const kitties = []

    const indices = [...Array(count).keys()];
    const kitties = indices.map(ind => ({
      id: ind+1,
      dna: dnas[ind],
      owner: owners[ind],
      market: market[ind]
    }));

    setKitties(kitties)
  }

  useEffect(fetchKitties, [api, keyring]);
  useEffect(populateKitties, [dnas, owners, market]);

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}
