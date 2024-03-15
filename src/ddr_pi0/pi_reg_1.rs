#[doc = "Register `PI_REG_1` reader"]
pub type R = crate::R<PiReg1Spec>;
#[doc = "Register `PI_REG_1` writer"]
pub type W = crate::W<PiReg1Spec>;
#[doc = "Field `PI_NORMAL_LVL_SEQ` reader - Enables the PI strategy that PI must finish all the pending leveling before it releases the DFI bus."]
pub type PiNormalLvlSeqR = crate::BitReader;
#[doc = "Field `PI_NORMAL_LVL_SEQ` writer - Enables the PI strategy that PI must finish all the pending leveling before it releases the DFI bus."]
pub type PiNormalLvlSeqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_INIT_LVL_EN` reader - Enables the initial leveling sequence after PI initialization procedure. Set to 1 to enable."]
pub type PiInitLvlEnR = crate::BitReader;
#[doc = "Field `PI_INIT_LVL_EN` writer - Enables the initial leveling sequence after PI initialization procedure. Set to 1 to enable."]
pub type PiInitLvlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TCMP_GAP` reader - Indicates DRAM timing in DFI clock cycles. Specifies the minimum gap between two commands. Used for guarding the timing from the last command of DDR controller and the first command of PI when DDR controller passes the control of the DFI bus to the PI."]
pub type PiTcmpGapR = crate::FieldReader<u16>;
#[doc = "Field `PI_TCMP_GAP` writer - Indicates DRAM timing in DFI clock cycles. Specifies the minimum gap between two commands. Used for guarding the timing from the last command of DDR controller and the first command of PI when DDR controller passes the control of the DFI bus to the PI."]
pub type PiTcmpGapW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enables the PI strategy that PI must finish all the pending leveling before it releases the DFI bus."]
    #[inline(always)]
    pub fn pi_normal_lvl_seq(&self) -> PiNormalLvlSeqR {
        PiNormalLvlSeqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the initial leveling sequence after PI initialization procedure. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_init_lvl_en(&self) -> PiInitLvlEnR {
        PiInitLvlEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Indicates DRAM timing in DFI clock cycles. Specifies the minimum gap between two commands. Used for guarding the timing from the last command of DDR controller and the first command of PI when DDR controller passes the control of the DFI bus to the PI."]
    #[inline(always)]
    pub fn pi_tcmp_gap(&self) -> PiTcmpGapR {
        PiTcmpGapR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the PI strategy that PI must finish all the pending leveling before it releases the DFI bus."]
    #[inline(always)]
    #[must_use]
    pub fn pi_normal_lvl_seq(&mut self) -> PiNormalLvlSeqW<PiReg1Spec> {
        PiNormalLvlSeqW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables the initial leveling sequence after PI initialization procedure. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_init_lvl_en(&mut self) -> PiInitLvlEnW<PiReg1Spec> {
        PiInitLvlEnW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Indicates DRAM timing in DFI clock cycles. Specifies the minimum gap between two commands. Used for guarding the timing from the last command of DDR controller and the first command of PI when DDR controller passes the control of the DFI bus to the PI."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcmp_gap(&mut self) -> PiTcmpGapW<PiReg1Spec> {
        PiTcmpGapW::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg1Spec;
impl crate::RegisterSpec for PiReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_1::R`](R) reader structure"]
impl crate::Readable for PiReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_1::W`](W) writer structure"]
impl crate::Writable for PiReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_1 to value 0"]
impl crate::Resettable for PiReg1Spec {
    const RESET_VALUE: u32 = 0;
}
