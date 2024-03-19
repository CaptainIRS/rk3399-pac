#[doc = "Register `FC_GMD_EN` reader"]
pub type R = crate::R<FcGmdEnSpec>;
#[doc = "Register `FC_GMD_EN` writer"]
pub type W = crate::W<FcGmdEnSpec>;
#[doc = "Field `GMDENABLETX` reader - Gamut Metadata packet transmission enable (1b)"]
pub type GmdenabletxR = crate::BitReader;
#[doc = "Field `GMDENABLETX` writer - Gamut Metadata packet transmission enable (1b)"]
pub type GmdenabletxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Gamut Metadata packet transmission enable (1b)"]
    #[inline(always)]
    pub fn gmdenabletx(&self) -> GmdenabletxR {
        GmdenabletxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gamut Metadata packet transmission enable (1b)"]
    #[inline(always)]
    #[must_use]
    pub fn gmdenabletx(&mut self) -> GmdenabletxW<FcGmdEnSpec> {
        GmdenabletxW::new(self, 0)
    }
}
#[doc = "Frame Composer GMD Packet Enable Register\n\nThis register enables Gamut metadata (GMD) packet transmission. Packets are inserted in\n\nthe incoming frame, starting in the line where active Vsync indication starts. After enable of\n\nGMD packets the outgoing packet is sent with no_current_gmd active indication until\n\nupdate GMD request is performed in the controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcGmdEnSpec;
impl crate::RegisterSpec for FcGmdEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_gmd_en::R`](R) reader structure"]
impl crate::Readable for FcGmdEnSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_gmd_en::W`](W) writer structure"]
impl crate::Writable for FcGmdEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_GMD_EN to value 0"]
impl crate::Resettable for FcGmdEnSpec {
    const RESET_VALUE: u8 = 0;
}
