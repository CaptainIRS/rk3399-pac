#[doc = "Register `PI_REG_117` reader"]
pub type R = crate::R<PiReg117Spec>;
#[doc = "Register `PI_REG_117` writer"]
pub type W = crate::W<PiReg117Spec>;
#[doc = "Field `PI_TCKEHDQS_F2` reader - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTckehdqsF2R = crate::FieldReader;
#[doc = "Field `PI_TCKEHDQS_F2` writer - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTckehdqsF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_WDQLVL_VREF_EN` reader - Indicates whether to do VREF training for non-initial WDQ leveling"]
pub type PiWdqlvlVrefEnR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_VREF_EN` writer - Indicates whether to do VREF training for non-initial WDQ leveling"]
pub type PiWdqlvlVrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_BST_NUM` reader - Indicates burst number for MPC, the maximum FIFO for LPDDR4 is 5."]
pub type PiWdqlvlBstNumR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_BST_NUM` writer - Indicates burst number for MPC, the maximum FIFO for LPDDR4 is 5."]
pub type PiWdqlvlBstNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tckehdqs_f2(&self) -> PiTckehdqsF2R {
        PiTckehdqsF2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Indicates whether to do VREF training for non-initial WDQ leveling"]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_en(&self) -> PiWdqlvlVrefEnR {
        PiWdqlvlVrefEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Indicates burst number for MPC, the maximum FIFO for LPDDR4 is 5."]
    #[inline(always)]
    pub fn pi_wdqlvl_bst_num(&self) -> PiWdqlvlBstNumR {
        PiWdqlvlBstNumR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Indicates the DRAM timing TCKEHDQS, minimum delay from CKE high to strobe high impedance. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tckehdqs_f2(&mut self) -> PiTckehdqsF2W<PiReg117Spec> {
        PiTckehdqsF2W::new(self, 0)
    }
    #[doc = "Bit 8 - Indicates whether to do VREF training for non-initial WDQ leveling"]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_en(&mut self) -> PiWdqlvlVrefEnW<PiReg117Spec> {
        PiWdqlvlVrefEnW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Indicates burst number for MPC, the maximum FIFO for LPDDR4 is 5."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_bst_num(&mut self) -> PiWdqlvlBstNumW<PiReg117Spec> {
        PiWdqlvlBstNumW::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 117\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_117::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_117::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg117Spec;
impl crate::RegisterSpec for PiReg117Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_117::R`](R) reader structure"]
impl crate::Readable for PiReg117Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_117::W`](W) writer structure"]
impl crate::Writable for PiReg117Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_117 to value 0"]
impl crate::Resettable for PiReg117Spec {
    const RESET_VALUE: u32 = 0;
}
