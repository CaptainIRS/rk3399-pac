#[doc = "Register `TX_INVID0` reader"]
pub type R = crate::R<TxInvid0Spec>;
#[doc = "Register `TX_INVID0` writer"]
pub type W = crate::W<TxInvid0Spec>;
#[doc = "Field `INTERNAL_DE_GENERATOR` reader - Internal data enable (DE) generator enable. If data\n\nenable is not available for the input video, set this\n\nbit to one to activate the internal data enable\n\ngenerator.\n\nAttention: This feature only works for input video\n\nmodes that have native repetition (such as, all CEA\n\nvideos). No desired pixel repetition can be used\n\nwith this feature because these configurations only\n\naffect the Frame Composer and not this block.\n\nThe DE Generator does not work for the following\n\nconditions:\n\nTransmission of video with CEA VIC 39\n\nTransmission of 3D video using the field\n\nalternative structure"]
pub type InternalDeGeneratorR = crate::BitReader;
#[doc = "Field `INTERNAL_DE_GENERATOR` writer - Internal data enable (DE) generator enable. If data\n\nenable is not available for the input video, set this\n\nbit to one to activate the internal data enable\n\ngenerator.\n\nAttention: This feature only works for input video\n\nmodes that have native repetition (such as, all CEA\n\nvideos). No desired pixel repetition can be used\n\nwith this feature because these configurations only\n\naffect the Frame Composer and not this block.\n\nThe DE Generator does not work for the following\n\nconditions:\n\nTransmission of video with CEA VIC 39\n\nTransmission of 3D video using the field\n\nalternative structure"]
pub type InternalDeGeneratorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Internal data enable (DE) generator enable. If data\n\nenable is not available for the input video, set this\n\nbit to one to activate the internal data enable\n\ngenerator.\n\nAttention: This feature only works for input video\n\nmodes that have native repetition (such as, all CEA\n\nvideos). No desired pixel repetition can be used\n\nwith this feature because these configurations only\n\naffect the Frame Composer and not this block.\n\nThe DE Generator does not work for the following\n\nconditions:\n\nTransmission of video with CEA VIC 39\n\nTransmission of 3D video using the field\n\nalternative structure"]
    #[inline(always)]
    pub fn internal_de_generator(&self) -> InternalDeGeneratorR {
        InternalDeGeneratorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Internal data enable (DE) generator enable. If data\n\nenable is not available for the input video, set this\n\nbit to one to activate the internal data enable\n\ngenerator.\n\nAttention: This feature only works for input video\n\nmodes that have native repetition (such as, all CEA\n\nvideos). No desired pixel repetition can be used\n\nwith this feature because these configurations only\n\naffect the Frame Composer and not this block.\n\nThe DE Generator does not work for the following\n\nconditions:\n\nTransmission of video with CEA VIC 39\n\nTransmission of 3D video using the field\n\nalternative structure"]
    #[inline(always)]
    #[must_use]
    pub fn internal_de_generator(&mut self) -> InternalDeGeneratorW<TxInvid0Spec> {
        InternalDeGeneratorW::new(self, 7)
    }
}
#[doc = "Video Input Mapping and Internal Data Enable Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_invid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_invid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
