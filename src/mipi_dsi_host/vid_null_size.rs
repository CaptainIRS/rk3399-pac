#[doc = "Register `VID_NULL_SIZE` reader"]
pub type R = crate::R<VidNullSizeSpec>;
#[doc = "Register `VID_NULL_SIZE` writer"]
pub type W = crate::W<VidNullSizeSpec>;
#[doc = "Field `VID_NULL_SIZE` reader - vid_null_size\n\nThis register configures the number of bytes inside a null packet.\n\nSetting it to 0 disables the null packets."]
pub type VidNullSizeR = crate::FieldReader<u16>;
#[doc = "Field `VID_NULL_SIZE` writer - vid_null_size\n\nThis register configures the number of bytes inside a null packet.\n\nSetting it to 0 disables the null packets."]
pub type VidNullSizeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - vid_null_size\n\nThis register configures the number of bytes inside a null packet.\n\nSetting it to 0 disables the null packets."]
    #[inline(always)]
    pub fn vid_null_size(&self) -> VidNullSizeR {
        VidNullSizeR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - vid_null_size\n\nThis register configures the number of bytes inside a null packet.\n\nSetting it to 0 disables the null packets."]
    #[inline(always)]
    #[must_use]
    pub fn vid_null_size(&mut self) -> VidNullSizeW<VidNullSizeSpec> {
        VidNullSizeW::new(self, 0)
    }
}
#[doc = "Null Packet Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_null_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_null_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidNullSizeSpec;
impl crate::RegisterSpec for VidNullSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_null_size::R`](R) reader structure"]
impl crate::Readable for VidNullSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_null_size::W`](W) writer structure"]
impl crate::Writable for VidNullSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_NULL_SIZE to value 0"]
impl crate::Resettable for VidNullSizeSpec {
    const RESET_VALUE: u32 = 0;
}
