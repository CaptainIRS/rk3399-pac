#[doc = "Register `DDR_DENALI_CTL_101` reader"]
pub type R = crate::R<DdrDenaliCtl101Spec>;
#[doc = "Register `DDR_DENALI_CTL_101` writer"]
pub type W = crate::W<DdrDenaliCtl101Spec>;
#[doc = "Field `LP_AUTO_ENTRY_EN` reader - Enable auto entry into each of the low power states when the associated idle timer expires. Bit (0) controls power-down, bit (1) controls self-refresh, bit (2) controls self-refresh with memory and controller clock gating, and bit (3) is reserved. Set each bit to 1 to enable."]
pub type LpAutoEntryEnR = crate::FieldReader;
#[doc = "Field `LP_AUTO_ENTRY_EN` writer - Enable auto entry into each of the low power states when the associated idle timer expires. Bit (0) controls power-down, bit (1) controls self-refresh, bit (2) controls self-refresh with memory and controller clock gating, and bit (3) is reserved. Set each bit to 1 to enable."]
pub type LpAutoEntryEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LP_AUTO_EXIT_EN` reader - Enable auto exit from each of the low power states when a read or write command enters the command queue. Bit (0) controls power-down, bit (1) controls lite self-refresh, and bit (2) controls deep self-refresh. Set each bit to 1 to enable."]
pub type LpAutoExitEnR = crate::FieldReader;
#[doc = "Field `LP_AUTO_EXIT_EN` writer - Enable auto exit from each of the low power states when a read or write command enters the command queue. Bit (0) controls power-down, bit (1) controls lite self-refresh, and bit (2) controls deep self-refresh. Set each bit to 1 to enable."]
pub type LpAutoExitEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LP_AUTO_MEM_GATE_EN` reader - Enable memory clock gating when entering a low power state via the auto low power counters. Bit (0) controls power-down, and bit (1) controls self-refresh. Set each bit to 1 to enable."]
pub type LpAutoMemGateEnR = crate::FieldReader;
#[doc = "Field `LP_AUTO_MEM_GATE_EN` writer - Enable memory clock gating when entering a low power state via the auto low power counters. Bit (0) controls power-down, and bit (1) controls self-refresh. Set each bit to 1 to enable."]
pub type LpAutoMemGateEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Enable auto entry into each of the low power states when the associated idle timer expires. Bit (0) controls power-down, bit (1) controls self-refresh, bit (2) controls self-refresh with memory and controller clock gating, and bit (3) is reserved. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn lp_auto_entry_en(&self) -> LpAutoEntryEnR {
        LpAutoEntryEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Enable auto exit from each of the low power states when a read or write command enters the command queue. Bit (0) controls power-down, bit (1) controls lite self-refresh, and bit (2) controls deep self-refresh. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn lp_auto_exit_en(&self) -> LpAutoExitEnR {
        LpAutoExitEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Enable memory clock gating when entering a low power state via the auto low power counters. Bit (0) controls power-down, and bit (1) controls self-refresh. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn lp_auto_mem_gate_en(&self) -> LpAutoMemGateEnR {
        LpAutoMemGateEnR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Enable auto entry into each of the low power states when the associated idle timer expires. Bit (0) controls power-down, bit (1) controls self-refresh, bit (2) controls self-refresh with memory and controller clock gating, and bit (3) is reserved. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_entry_en(&mut self) -> LpAutoEntryEnW<DdrDenaliCtl101Spec> {
        LpAutoEntryEnW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Enable auto exit from each of the low power states when a read or write command enters the command queue. Bit (0) controls power-down, bit (1) controls lite self-refresh, and bit (2) controls deep self-refresh. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_exit_en(&mut self) -> LpAutoExitEnW<DdrDenaliCtl101Spec> {
        LpAutoExitEnW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Enable memory clock gating when entering a low power state via the auto low power counters. Bit (0) controls power-down, and bit (1) controls self-refresh. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_mem_gate_en(&mut self) -> LpAutoMemGateEnW<DdrDenaliCtl101Spec> {
        LpAutoMemGateEnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_101::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_101::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl101Spec;
impl crate::RegisterSpec for DdrDenaliCtl101Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_101::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl101Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_101::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl101Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_101 to value 0"]
impl crate::Resettable for DdrDenaliCtl101Spec {
    const RESET_VALUE: u32 = 0;
}
