import { useCallback, useEffect, useRef } from "react";
import styles from "./SplashScreen.module.css";
import themeMp3 from "../../assets/theme.mp3";

interface SplashScreenProps {
  onDismiss: () => void;
}

export default function SplashScreen({ onDismiss }: SplashScreenProps) {
  const audioRef = useRef<HTMLAudioElement>(null);
  const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  // ponytail: no auto-dismiss timer — splash stays until the user acts (button/click/key)
  const stopAndDismiss = useCallback(() => {
    const a = audioRef.current;
    if (a) {
      a.pause();
      a.currentTime = 0;
    }
    onDismiss();
  }, [onDismiss]);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === "Enter" || e.key === " " || e.key === "Escape") {
        e.preventDefault();
        stopAndDismiss();
      }
    },
    [stopAndDismiss]
  );

  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleKeyDown]);

  useEffect(() => {
    if (audioRef.current) audioRef.current.volume = 0.4;
  }, []);

  useEffect(() => {
    audioRef.current?.play().catch(() => {});
    return () => {
      const a = audioRef.current;
      if (a) {
        a.pause();
        a.currentTime = 0;
      }
    };
  }, []);

  return (
    <div
      role="dialog"
      aria-label="Game intro splash screen"
      aria-modal="true"
      className={styles.splash}
      onClick={stopAndDismiss}
    >
      <div className={styles.content}>
        <img
          src="/splash-logo.png"
          alt="Cheesy Money"
          className={styles.logo}
        />

        <img
          src={reducedMotion ? "/splash-mascot.png" : "/splash-mascot-anim.webp"}
          alt=""
          aria-hidden="true"
          className={styles.mascot}
        />

        <span className={styles.pressAnyKey}>PRESS ANY KEY</span>

        <button
          type="button"
          className={styles.skipBtn}
          onClick={(e) => {
            e.stopPropagation();
            stopAndDismiss();
          }}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.stopPropagation();
              stopAndDismiss();
            }
          }}
          autoFocus
        >
          [ SKIP INTRO ]
        </button>
      </div>

      <audio ref={audioRef} src={themeMp3} loop />
    </div>
  );
}
