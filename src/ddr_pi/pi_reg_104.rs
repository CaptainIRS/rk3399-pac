#[doc = "Register `PI_REG_104` reader"]
pub type R = crate::R<PiReg104Spec>;
#[doc = "Register `PI_REG_104` writer"]
pub type W = crate::W<PiReg104Spec>;
#[doc = "Field `PI_TCAENT_F2` reader - Indicates DRAM TCAENT value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTcaentF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TCAENT_F2` writer - Indicates DRAM TCAENT value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
pub type PiTcaentF2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PI_TCAEXT` reader - Indicates DRAM TCAEXT value in cycles."]
pub type PiTcaextR = crate::FieldReader;
#[doc = "Field `PI_TCAEXT` writer - Indicates DRAM TCAEXT value in cycles."]
pub type PiTcaextW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_CA_TRAIN_VREF_EN` reader - Indicates whether to do VREF training during non-power-on-initial CA training or not. Set to 1 to go through the VREF from start-point for non-power-on-initial CA training. Set to 0 to not update VREF for non-power-on-initial CA training."]
pub type PiCaTrainVrefEnR = crate::BitReader;
#[doc = "Field `PI_CA_TRAIN_VREF_EN` writer - Indicates whether to do VREF training during non-power-on-initial CA training or not. Set to 1 to go through the VREF from start-point for non-power-on-initial CA training. Set to 0 to not update VREF for non-power-on-initial CA training."]
pub type PiCaTrainVrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Indicates DRAM TCAENT value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_tcaent_f2(&self) -> PiTcaentF2R {
        PiTcaentF2R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:20 - Indicates DRAM TCAEXT value in cycles."]
    #[inline(always)]
    pub fn pi_tcaext(&self) -> PiTcaextR {
        PiTcaextR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Indicates whether to do VREF training during non-power-on-initial CA training or not. Set to 1 to go through the VREF from start-point for non-power-on-initial CA training. Set to 0 to not update VREF for non-power-on-initial CA training."]
    #[inline(always)]
    pub fn pi_ca_train_vref_en(&self) -> PiCaTrainVrefEnR {
        PiCaTrainVrefEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Indicates DRAM TCAENT value in cycles. The suffix \"_f2\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcaent_f2(&mut self) -> PiTcaentF2W<PiReg104Spec> {
        PiTcaentF2W::new(self, 0)
    }
    #[doc = "Bits 16:20 - Indicates DRAM TCAEXT value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcaext(&mut self) -> PiTcaextW<PiReg104Spec> {
        PiTcaextW::new(self, 16)
    }
    #[doc = "Bit 24 - Indicates whether to do VREF training during non-power-on-initial CA training or not. Set to 1 to go through the VREF from start-point for non-power-on-initial CA training. Set to 0 to not update VREF for non-power-on-initial CA training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ca_train_vref_en(&mut self) -> PiCaTrainVrefEnW<PiReg104Spec> {
        PiCaTrainVrefEnW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_104::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_104::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg104Spec;
impl crate::RegisterSpec for PiReg104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_104::R`](R) reader structure"]
impl crate::Readable for PiReg104Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_104::W`](W) writer structure"]
impl crate::Writable for PiReg104Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_104 to value 0"]
impl crate::Resettable for PiReg104Spec {
    const RESET_VALUE: u32 = 0;
}
