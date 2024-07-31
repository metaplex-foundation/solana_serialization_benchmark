import { generateSigner } from "@metaplex-foundation/umi";
// eslint-disable-next-line import/no-extraneous-dependencies
import test from "ava";
import { existsSync, readFileSync, writeFileSync } from "fs";
import { createBasicNone, createCollectionNone, updateBasicNone, updateCollectionNone } from '../src';
import { createUmi } from "./_setup";

test('Create:Basic:None', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createBasicNone(umi, { address }).sendAndConfirm(umi);

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
    if (existsSync("../../page/output.json")) {
        output = JSON.parse(readFileSync("../../page/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../page/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Read:Basic:None', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createBasicNone(umi, { address }).sendAndConfirm(umi);

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
    if (existsSync("../../page/output.json")) {
        output = JSON.parse(readFileSync("../../page/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../page/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Update:Basic:None', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    await createBasicNone(umi, { address }).sendAndConfirm(umi);
    const tx = await updateBasicNone(umi, { address: address.publicKey }).sendAndConfirm(umi);

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
    if (existsSync("../../page/output.json")) {
        output = JSON.parse(readFileSync("../../page/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../page/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Create:Collection:None', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createCollectionNone(umi, { address }).sendAndConfirm(umi);

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
    if (existsSync("../../page/output.json")) {
        output = JSON.parse(readFileSync("../../page/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../page/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Read:Collection:None', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createCollectionNone(umi, { address }).sendAndConfirm(umi);

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
    if (existsSync("../../page/output.json")) {
        output = JSON.parse(readFileSync("../../page/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../page/output.json", JSON.stringify(output, null, 2));

    t.pass();
});

test('Update:Collection:None', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    await createCollectionNone(umi, { address }).sendAndConfirm(umi);
    const tx = await updateCollectionNone(umi, { address: address.publicKey }).sendAndConfirm(umi);

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
    if (existsSync("../../page/output.json")) {
        output = JSON.parse(readFileSync("../../page/output.json", 'utf-8'));
    }

    // Push the result to the array
    output.push(cuResult);
    output.push(spaceResult);
    // Write the array to output.json
    writeFileSync("../../page/output.json", JSON.stringify(output, null, 2));

    t.pass();
});