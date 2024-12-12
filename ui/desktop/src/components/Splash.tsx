import React, { useState, useEffect } from 'react';
import GooseSplashLogo from './GooseSplashLogo';
import SplashPills from './SplashPills';
import { featureFlags, type FeatureFlags } from '../featureFlags';

export default function Splash({ append }) {
  const [flags, setFlags] = useState<FeatureFlags>(featureFlags.getFlags());

  // Update all feature flags when they change
  useEffect(() => {
    const updateInterval = setInterval(() => {
      const currentFlags = featureFlags.getFlags();
      // Deep comparison to avoid unnecessary updates
      if (JSON.stringify(currentFlags) !== JSON.stringify(flags)) {
        setFlags(currentFlags);
      }
    }, 1000);

    return () => clearInterval(updateInterval);
  }, [flags]);

  return (
    <div className="h-full flex flex-col items-center justify-center">
      <div className="flex flex-1" />
      <div className="flex items-center">
        <GooseSplashLogo />
        <span className="ask-goose-type goose-text dark:goose-text-dark ml-[8px]">ask<br />goose</span>
      </div>
      <div className={`mt-[10px] w-[198px] h-[17px] py-2 flex-col justify-center items-start inline-flex`}>
        <div className="self-stretch h-px bg-black/5 dark:bg-white/5 rounded-sm" />
      </div>
      <div
        className="w-[312px] px-16 py-4 text-14 text-center text-splash-pills-text dark:text-splash-pills-text-dark whitespace-nowrap cursor-pointer bg-prev-goose-gradient dark:bg-dark-prev-goose-gradient text-prev-goose-text dark:text-prev-goose-text-dark rounded-[14px] inline-block hover:scale-[1.02] transition-all duration-150"
        onClick={async () => {
          const message = {
            content: flags.whatCanGooseDoText,
            role: "user",
          };
          await append(message);
        }}
      >
        {flags.whatCanGooseDoText}
      </div>
      <div className="flex flex-1" />
      <div className={`mt-[10px] w-[198px] h-[17px] py-2 flex-col justify-center items-start inline-flex`}>
        <div className="self-stretch h-px bg-black/5 dark:bg-white/5 rounded-sm" />
      </div>
      <div className="flex items-center p-4">
        <SplashPills append={append} />
      </div>
    </div>
  )
}