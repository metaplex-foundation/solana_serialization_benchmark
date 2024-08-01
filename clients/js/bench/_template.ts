// For new Serialization libraries, copy and paste this _template File. Rename the file
// to the name of the serialization library. Replace the "SERDESLIB" with the name of the
// serialization library.
import { generateSigner } from "@metaplex-foundation/umi";
// eslint-disable-next-line import/no-extraneous-dependencies
import test from "ava";
import { existsSync, readFileSync, writeFileSync } from "fs";
import { createBasicSERDESLIB, createCollectionSERDESLIB, updateBasicSERDESLIB, updateCollectionSERDESLIB } from '../src';
import { createUmi } from "./_setup";

test('Create:Basic:SERDESLIB', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createBasicSERDESLIB(umi, { address }).sendAndConfirm(umi);

    const compute = Number((await umi.rpc.getTransaction(tx.signature))?.meta.computeUnitsConsumed);
    const account = await umi.rpc.getAccount(address.publicKey);
    const space = account.exists ? account.data.length : 0;

    const cuResult = {
        name: `Compute:${t.title}`,
        unit: "Compute Units",
        value: compute,
    }

    const spaceResult = {
        name: `Space:${t.title}`,
        unit: "Bytes",
        value: space,
    }

    // Read the results array from output.json
    let output = [];
    if (existsSync("../../docs/output.json")) {
        output = JSON.parse(readFileSync("../../docs/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../docs/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Read:Basic:SERDESLIB', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createBasicSERDESLIB(umi, { address }).sendAndConfirm(umi);

    // Then an account was created with the correct data.
    const compute = Number((await umi.rpc.getTransaction(tx.signature))?.meta.computeUnitsConsumed);
    const account = await umi.rpc.getAccount(address.publicKey);
    const space = account.exists ? account.data.length : 0;

    const cuResult = {
        name: `Compute:${t.title}`,
        unit: "Compute Units",
        value: compute,
    }

    const spaceResult = {
        name: `Space:${t.title}`,
        unit: "Bytes",
        value: space,
    }

    // Read the results array from output.json
    let output = [];
    if (existsSync("../../docs/output.json")) {
        output = JSON.parse(readFileSync("../../docs/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../docs/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Update:Basic:SERDESLIB', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    await createBasicSERDESLIB(umi, { address }).sendAndConfirm(umi);
    const tx = await updateBasicSERDESLIB(umi, { address: address.publicKey }).sendAndConfirm(umi);

    const compute = Number((await umi.rpc.getTransaction(tx.signature))?.meta.computeUnitsConsumed);
    const account = await umi.rpc.getAccount(address.publicKey);
    const space = account.exists ? account.data.length : 0;

    const cuResult = {
        name: `Compute:${t.title}`,
        unit: "Compute Units",
        value: compute,
    }

    const spaceResult = {
        name: `Space:${t.title}`,
        unit: "Bytes",
        value: space,
    }

    // Read the results array from output.json
    let output = [];
    if (existsSync("../../docs/output.json")) {
        output = JSON.parse(readFileSync("../../docs/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../docs/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Create:Collection:SERDESLIB', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createCollectionSERDESLIB(umi, { address }).sendAndConfirm(umi);

    const compute = Number((await umi.rpc.getTransaction(tx.signature))?.meta.computeUnitsConsumed);
    const account = await umi.rpc.getAccount(address.publicKey);
    const space = account.exists ? account.data.length : 0;

    const cuResult = {
        name: `Compute:${t.title}`,
        unit: "Compute Units",
        value: compute,
    }

    const spaceResult = {
        name: `Space:${t.title}`,
        unit: "Bytes",
        value: space,
    }

    // Read the results array from output.json
    let output = [];
    if (existsSync("../../docs/output.json")) {
        output = JSON.parse(readFileSync("../../docs/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../docs/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Read:Collection:SERDESLIB', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createCollectionSERDESLIB(umi, { address }).sendAndConfirm(umi);

    // Then an account was created with the correct data.
    const compute = Number((await umi.rpc.getTransaction(tx.signature))?.meta.computeUnitsConsumed);
    const account = await umi.rpc.getAccount(address.publicKey);
    const space = account.exists ? account.data.length : 0;

    const cuResult = {
        name: `Compute:${t.title}`,
        unit: "Compute Units",
        value: compute,
    }

    const spaceResult = {
        name: `Space:${t.title}`,
        unit: "Bytes",
        value: space,
    }

    // Read the results array from output.json
    let output = [];
    if (existsSync("../../docs/output.json")) {
        output = JSON.parse(readFileSync("../../docs/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../docs/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Update:Collection:SERDESLIB', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    await createCollectionSERDESLIB(umi, { address }).sendAndConfirm(umi);
    const tx = await updateCollectionSERDESLIB(umi, { address: address.publicKey }).sendAndConfirm(umi);

    const compute = Number((await umi.rpc.getTransaction(tx.signature))?.meta.computeUnitsConsumed);
    const account = await umi.rpc.getAccount(address.publicKey);
    const space = account.exists ? account.data.length : 0;

    const cuResult = {
        name: `Compute:${t.title}`,
        unit: "Compute Units",
        value: compute,
    }

    const spaceResult = {
        name: `Space:${t.title}`,
        unit: "Bytes",
        value: space,
    }

    // Read the results array from output.json
    let output = [];
    if (existsSync("../../docs/output.json")) {
        output = JSON.parse(readFileSync("../../docs/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../docs/output.json", JSON.stringify(output, null, 2));

    t.pass();
});