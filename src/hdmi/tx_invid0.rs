#[doc = "Register `TX_INVID0` reader"]
pub type R = crate::R<TxInvid0Spec>;
#[doc = "Register `TX_INVID0` writer"]
pub type W = crate::W<TxInvid0Spec>;
#[doc = "Field `INTERNAL_DE_GENERATOR` reader - Internal data enable (DE) generator enable. If data enable is not available for the input video, set this bit to one to activate the internal data enable generator. Attention: This feature only works for input video modes that have native repetition (such as, all CEA videos). No desired pixel repetition can be used with this feature because these configurations only affect the Frame Composer and not this block. The DE Generator does not work for the following conditions: Transmission of video with CEA VIC 39 Transmission of 3D video using the field alternative structure"]
pub type InternalDeGeneratorR = crate::BitReader;
#[doc = "Field `INTERNAL_DE_GENERATOR` writer - Internal data enable (DE) generator enable. If data enable is not available for the input video, set this bit to one to activate the internal data enable generator. Attention: This feature only works for input video modes that have native repetition (such as, all CEA videos). No desired pixel repetition can be used with this feature because these configurations only affect the Frame Composer and not this block. The DE Generator does not work for the following conditions: Transmission of video with CEA VIC 39 Transmission of 3D video using the field alternative structure"]
pub type InternalDeGeneratorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Internal data enable (DE) generator enable. If data enable is not available for the input video, set this bit to one to activate the internal data enable generator. Attention: This feature only works for input video modes that have native repetition (such as, all CEA videos). No desired pixel repetition can be used with this feature because these configurations only affect the Frame Composer and not this block. The DE Generator does not work for the following conditions: Transmission of video with CEA VIC 39 Transmission of 3D video using the field alternative structure"]
    #[inline(always)]
    pub fn internal_de_generator(&self) -> InternalDeGeneratorR {
        InternalDeGeneratorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Internal data enable (DE) generator enable. If data enable is not available for the input video, set this bit to one to activate the internal data enable generator. Attention: This feature only works for input video modes that have native repetition (such as, all CEA videos). No desired pixel repetition can be used with this feature because these configurations only affect the Frame Composer and not this block. The DE Generator does not work for the following conditions: Transmission of video with CEA VIC 39 Transmission of 3D video using the field alternative structure"]
    #[inline(always)]
    #[must_use]
    pub fn internal_de_generator(&mut self) -> InternalDeGeneratorW<TxInvid0Spec> {
        InternalDeGeneratorW::new(self, 7)
    }
}
#[doc = "Internal data enable (DE) generator enable. If data enable is not available for the input video, set this bit to one to activate the internal data enable generator. Attention: This feature only works for input video modes that have native repetition (such as, all CEA videos). No desired pixel repetition can be used with this feature because these configurations only affect the Frame Composer and not this block. The DE Generator does not work for the following conditions: Transmission of video with CEA VIC 39 Transmission of 3D video using the field alternative structure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_invid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_invid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxInvid0Spec;
impl crate::RegisterSpec for TxInvid0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_invid0::R`](R) reader structure"]
impl crate::Readable for TxInvid0Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_invid0::W`](W) writer structure"]
impl crate::Writable for TxInvid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TX_INVID0 to value 0"]
impl crate::Resettable for TxInvid0Spec {
    const RESET_VALUE: u8 = 0;
}
