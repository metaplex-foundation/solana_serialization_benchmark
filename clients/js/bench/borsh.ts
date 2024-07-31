import { generateSigner } from "@metaplex-foundation/umi";
// eslint-disable-next-line import/no-extraneous-dependencies
import test from "ava";
import { existsSync, readFileSync, writeFileSync } from "fs";
import { createBasicBorsh, createCollectionBorsh, updateBasicBorsh, updateCollectionBorsh } from '../src';
import { createUmi } from "./_setup";

test('Create:Basic:Borsh', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createBasicBorsh(umi, { address }).sendAndConfirm(umi);

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

test('Read:Basic:Borsh', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createBasicBorsh(umi, { address }).sendAndConfirm(umi);

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

test('Update:Basic:Borsh', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    await createBasicBorsh(umi, { address }).sendAndConfirm(umi, { confirm: { commitment: 'finalized' } });
    const tx = await updateBasicBorsh(umi, { address: address.publicKey }).sendAndConfirm(umi);

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

test('Create:Collection:Borsh', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createCollectionBorsh(umi, { address }).sendAndConfirm(umi);

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

test('Read:Collection:Borsh', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    const tx = await createCollectionBorsh(umi, { address }).sendAndConfirm(umi);

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

test('Update:Collection:Borsh', async (t) => {
    // Given an Umi instance and a new signer.
    const umi = await createUmi();
    const address = generateSigner(umi);

    // When we create a new account.
    await createCollectionBorsh(umi, { address }).sendAndConfirm(umi);
    const tx = await updateCollectionBorsh(umi, { address: address.publicKey }).sendAndConfirm(umi, { send: { skipPreflight: true } });

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