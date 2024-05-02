#[doc = "Register `MI_INIT` writer"]
pub type W = crate::W<MiInitSpec>;
#[doc = "Field `mi_skip` writer - Skip of current or next starting main picture:\n\nAborts writing of main picture image data of the\n\ncurrent frame to RAM (after the current burst transmission\n\nhas been completed). Further main picture data up to the\n\nend of the current frame are discarded.\n\n\n\nNo further makroblock line interrupt (mblk_line), no\n\nwrap around interrupt for main picture (wrap_mp_y/cb/cr) and no fill level interrupt (fill_mp_y)\n\nare generated.\n\n\n\nSkip does not affect the generation of the main path\n\nframe end interrupt (mp_frame_end).\n\nSkip does not affect the processing of self picture and\n\nits corresponding interrupts namely the self path frame\n\nend interrupt (sp_frame_end).\n\n\n\nThe byte counter (register MI_BYTE_CNT) is not\n\naffected. It produces the correct number of JPEG or RAW\n\ndata bytes at the end of the current (skipped) frame.\n\nAfter a skip has been performed the offset counter for\n\nthe main picture at the start of the following frame are set\n\ndepending on the bit init_offset_en in register MI_CTRL:\n\nSkip restart mode (init_offset_en = 0)\n\nThe offset counters of the main picture are restarted\n\nat the old start values of the previous skipped frame.\n\nSkip init mode (init_offset_en = 1)\n\nThe offset counters of the main picture are initialized\n\nwith the register contents of the offset counter init\n\nregisters without any additional forced software update or\n\nautomatic config update."]
pub type MiSkipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mi_cfg_upd` writer - Forced configuration update. Leads to an immediate\n\nupdate of the shadow registers.\n\nDepending on the two init enable bits in the MI_CTRL\n\nregister (init_offset_en and init_base_en) the offset\n\ncounter, base address and buffer size shadow registers\n\nare also updated."]
pub type MiCfgUpdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Skip of current or next starting main picture:\n\nAborts writing of main picture image data of the\n\ncurrent frame to RAM (after the current burst transmission\n\nhas been completed). Further main picture data up to the\n\nend of the current frame are discarded.\n\n\n\nNo further makroblock line interrupt (mblk_line), no\n\nwrap around interrupt for main picture (wrap_mp_y/cb/cr) and no fill level interrupt (fill_mp_y)\n\nare generated.\n\n\n\nSkip does not affect the generation of the main path\n\nframe end interrupt (mp_frame_end).\n\nSkip does not affect the processing of self picture and\n\nits corresponding interrupts namely the self path frame\n\nend interrupt (sp_frame_end).\n\n\n\nThe byte counter (register MI_BYTE_CNT) is not\n\naffected. It produces the correct number of JPEG or RAW\n\ndata bytes at the end of the current (skipped) frame.\n\nAfter a skip has been performed the offset counter for\n\nthe main picture at the start of the following frame are set\n\ndepending on the bit init_offset_en in register MI_CTRL:\n\nSkip restart mode (init_offset_en = 0)\n\nThe offset counters of the main picture are restarted\n\nat the old start values of the previous skipped frame.\n\nSkip init mode (init_offset_en = 1)\n\nThe offset counters of the main picture are initialized\n\nwith the register contents of the offset counter init\n\nregisters without any additional forced software update or\n\nautomatic config update."]
    #[inline(always)]
    #[must_use]
    pub fn mi_skip(&mut self) -> MiSkipW<MiInitSpec> {
        MiSkipW::new(self, 2)
    }
    #[doc = "Bit 4 - Forced configuration update. Leads to an immediate\n\nupdate of the shadow registers.\n\nDepending on the two init enable bits in the MI_CTRL\n\nregister (init_offset_en and init_base_en) the offset\n\ncounter, base address and buffer size shadow registers\n\nare also updated."]
    #[inline(always)]
    #[must_use]
    pub fn mi_cfg_upd(&mut self) -> MiCfgUpdW<MiInitSpec> {
        MiCfgUpdW::new(self, 4)
    }
}
#[doc = "Control register for address init and skip function\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_init::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiInitSpec;
impl crate::RegisterSpec for MiInitSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mi_init::W`](W) writer structure"]
impl crate::Writable for MiInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_INIT to value 0"]
impl crate::Resettable for MiInitSpec {
    const RESET_VALUE: u32 = 0;
}
