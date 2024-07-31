/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { PublicKey } from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  map,
  publicKey as publicKeySerializer,
  set,
  struct,
} from '@metaplex-foundation/umi/serializers';

export type CollectionTypes = {
  vecPublicKey: Array<PublicKey>;
  hsetPublicKey: Set<PublicKey>;
  bsetPublicKey: Set<PublicKey>;
  hmapPublicKey: Map<PublicKey, PublicKey>;
  bmapPublicKey: Map<PublicKey, PublicKey>;
};

export type CollectionTypesArgs = CollectionTypes;

export function getCollectionTypesSerializer(): Serializer<
  CollectionTypesArgs,
  CollectionTypes
> {
  return struct<CollectionTypes>(
    [
      ['vecPublicKey', array(publicKeySerializer())],
      ['hsetPublicKey', set(publicKeySerializer())],
      ['bsetPublicKey', set(publicKeySerializer())],
      ['hmapPublicKey', map(publicKeySerializer(), publicKeySerializer())],
      ['bmapPublicKey', map(publicKeySerializer(), publicKeySerializer())],
    ],
    { description: 'CollectionTypes' }
  ) as Serializer<CollectionTypesArgs, CollectionTypes>;
}