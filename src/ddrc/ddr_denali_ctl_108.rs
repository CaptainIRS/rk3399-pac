#[doc = "Register `DDR_DENALI_CTL_108` reader"]
pub type R = crate::R<DdrDenaliCtl108Spec>;
#[doc = "Register `DDR_DENALI_CTL_108` writer"]
pub type W = crate::W<DdrDenaliCtl108Spec>;
#[doc = "Field `DFS_ENABLE` reader - Enable hardware dynamic frequency scaling. Set to 1 to enable."]
pub type DfsEnableR = crate::BitReader;
#[doc = "Field `DFS_ENABLE` writer - Enable hardware dynamic frequency scaling. Set to 1 to enable."]
pub type DfsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS_DLL_OFF` reader - Defines if the memory DLL must be off for the associated frequency set. Bit (0) corresponds to frequency set 0, bit (1) corresponds to frequency set 1, etc. Set each bit to 1 to require DLL off."]
pub type DfsDllOffR = crate::FieldReader;
#[doc = "Field `DFS_DLL_OFF` writer - Defines if the memory DLL must be off for the associated frequency set. Bit (0) corresponds to frequency set 0, bit (1) corresponds to frequency set 1, etc. Set each bit to 1 to require DLL off."]
pub type DfsDllOffW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 16 - Enable hardware dynamic frequency scaling. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_enable(&self) -> DfsEnableR {
        DfsEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Defines if the memory DLL must be off for the associated frequency set. Bit (0) corresponds to frequency set 0, bit (1) corresponds to frequency set 1, etc. Set each bit to 1 to require DLL off."]
    #[inline(always)]
    pub fn dfs_dll_off(&self) -> DfsDllOffR {
        DfsDllOffR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Enable hardware dynamic frequency scaling. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_enable(&mut self) -> DfsEnableW<DdrDenaliCtl108Spec> {
        DfsEnableW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Defines if the memory DLL must be off for the associated frequency set. Bit (0) corresponds to frequency set 0, bit (1) corresponds to frequency set 1, etc. Set each bit to 1 to require DLL off."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_dll_off(&mut self) -> DfsDllOffW<DdrDenaliCtl108Spec> {
        DfsDllOffW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_108::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_108::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl108Spec;
impl crate::RegisterSpec for DdrDenaliCtl108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_108::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl108Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_108::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl108Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_108 to value 0"]
impl crate::Resettable for DdrDenaliCtl108Spec {
    const RESET_VALUE: u32 = 0;
}
