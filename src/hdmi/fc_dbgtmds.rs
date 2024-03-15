#[doc = "Register `FC_DBGTMDS[%s]` reader"]
pub type R = crate::R<FcDbgtmdsSpec>;
#[doc = "Register `FC_DBGTMDS[%s]` writer"]
pub type W = crate::W<FcDbgtmdsSpec>;
#[doc = "Field `FC_DBGTMDS` reader - Frame Composer TMDS Data Channel 0 Register"]
pub type FcDbgtmdsR = crate::FieldReader;
#[doc = "Field `FC_DBGTMDS` writer - Frame Composer TMDS Data Channel 0 Register"]
pub type FcDbgtmdsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer TMDS Data Channel 0 Register"]
    #[inline(always)]
    pub fn fc_dbgtmds(&self) -> FcDbgtmdsR {
        FcDbgtmdsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer TMDS Data Channel 0 Register"]
    #[inline(always)]
    #[must_use]
    pub fn fc_dbgtmds(&mut self) -> FcDbgtmdsW<FcDbgtmdsSpec> {
        FcDbgtmdsW::new(self, 0)
    }
}
#[doc = "Frame Composer TMDS Data Channel 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgtmds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgtmds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDbgtmdsSpec;
impl crate::RegisterSpec for FcDbgtmdsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_dbgtmds::R`](R) reader structure"]
impl crate::Readable for FcDbgtmdsSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_dbgtmds::W`](W) writer structure"]
impl crate::Writable for FcDbgtmdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DBGTMDS[%s]
to value 0"]
impl crate::Resettable for FcDbgtmdsSpec {
    const RESET_VALUE: u8 = 0;
}
