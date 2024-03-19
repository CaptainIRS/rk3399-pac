#[doc = "Register `AUX_CH_CTL_1` reader"]
pub type R = crate::R<AuxChCtl1Spec>;
#[doc = "Register `AUX_CH_CTL_1` writer"]
pub type W = crate::W<AuxChCtl1Spec>;
#[doc = "Field `AUX_TX_COMM` reader - Register control AUX CH transaction \n\ncommand."]
pub type AuxTxCommR = crate::FieldReader;
#[doc = "Field `AUX_TX_COMM` writer - Register control AUX CH transaction \n\ncommand."]
pub type AuxTxCommW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AUX_LENGTH` reader - Register control AUX CH transaction length."]
pub type AuxLengthR = crate::FieldReader;
#[doc = "Field `AUX_LENGTH` writer - Register control AUX CH transaction length."]
pub type AuxLengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Register control AUX CH transaction \n\ncommand."]
    #[inline(always)]
    pub fn aux_tx_comm(&self) -> AuxTxCommR {
        AuxTxCommR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Register control AUX CH transaction length."]
    #[inline(always)]
    pub fn aux_length(&self) -> AuxLengthR {
        AuxLengthR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Register control AUX CH transaction \n\ncommand."]
    #[inline(always)]
    #[must_use]
    pub fn aux_tx_comm(&mut self) -> AuxTxCommW<AuxChCtl1Spec> {
        AuxTxCommW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Register control AUX CH transaction length."]
    #[inline(always)]
    #[must_use]
    pub fn aux_length(&mut self) -> AuxLengthW<AuxChCtl1Spec> {
        AuxLengthW::new(self, 4)
    }
}
#[doc = "DP AUX Channel Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_ch_ctl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_ch_ctl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxChCtl1Spec;
impl crate::RegisterSpec for AuxChCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_ch_ctl_1::R`](R) reader structure"]
impl crate::Readable for AuxChCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`aux_ch_ctl_1::W`](W) writer structure"]
impl crate::Writable for AuxChCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets AUX_CH_CTL_1 to value 0"]
impl crate::Resettable for AuxChCtl1Spec {
    const RESET_VALUE: u32 = 0;
}
