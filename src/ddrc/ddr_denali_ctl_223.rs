#[doc = "Register `DDR_DENALI_CTL_223` reader"]
pub type R = crate::R<DdrDenaliCtl223Spec>;
#[doc = "Register `DDR_DENALI_CTL_223` writer"]
pub type W = crate::W<DdrDenaliCtl223Spec>;
#[doc = "Field `SWLVL_LOAD` writer - User request to load delays and execute software leveling. Set to 1 to trigger. WRITE-ONLY"]
pub type SwlvlLoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWLVL_START` writer - User request to initiate software leveling of type in the SW_LEVELING_MODE parameter. Set to 1 to trigger. WRITE-ONLY"]
pub type SwlvlStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWLVL_EXIT` writer - User request to exit software leveling. Set to 1 to exit. WRITE- ONLY"]
pub type SwlvlExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWLVL_OP_DONE` reader - Signals that software leveling is currently in progress. Value of 1 indicates operation complete. READ-ONLY"]
pub type SwlvlOpDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 24 - Signals that software leveling is currently in progress. Value of 1 indicates operation complete. READ-ONLY"]
    #[inline(always)]
    pub fn swlvl_op_done(&self) -> SwlvlOpDoneR {
        SwlvlOpDoneR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User request to load delays and execute software leveling. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn swlvl_load(&mut self) -> SwlvlLoadW<DdrDenaliCtl223Spec> {
        SwlvlLoadW::new(self, 0)
    }
    #[doc = "Bit 8 - User request to initiate software leveling of type in the SW_LEVELING_MODE parameter. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn swlvl_start(&mut self) -> SwlvlStartW<DdrDenaliCtl223Spec> {
        SwlvlStartW::new(self, 8)
    }
    #[doc = "Bit 16 - User request to exit software leveling. Set to 1 to exit. WRITE- ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn swlvl_exit(&mut self) -> SwlvlExitW<DdrDenaliCtl223Spec> {
        SwlvlExitW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_223::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_223::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl223Spec;
impl crate::RegisterSpec for DdrDenaliCtl223Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_223::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl223Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_223::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl223Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_223 to value 0"]
impl crate::Resettable for DdrDenaliCtl223Spec {
    const RESET_VALUE: u32 = 0;
}
