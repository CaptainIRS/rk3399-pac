#[doc = "Register `DENALI_CTL_124` reader"]
pub type R = crate::R<DenaliCtl124Spec>;
#[doc = "Register `DENALI_CTL_124` writer"]
pub type W = crate::W<DenaliCtl124Spec>;
#[doc = "Field `TCKFSPE_F0` reader - JEDEC TCKFSPE, the valid clock requirement after entering SDP change."]
pub type TckfspeF0R = crate::FieldReader;
#[doc = "Field `TCKFSPE_F0` writer - JEDEC TCKFSPE, the valid clock requirement after entering SDP change."]
pub type TckfspeF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKFSPX_F0` reader - JEDEC TCKFSPX, the valid clock requirement before 1st valid command after FSP change."]
pub type TckfspxF0R = crate::FieldReader;
#[doc = "Field `TCKFSPX_F0` writer - JEDEC TCKFSPX, the valid clock requirement before 1st valid command after FSP change."]
pub type TckfspxF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TVREF_LONG_F0` reader - JEDEC TVREF, design will always use the long value."]
pub type TvrefLongF0R = crate::FieldReader<u16>;
#[doc = "Field `TVREF_LONG_F0` writer - JEDEC TVREF, design will always use the long value."]
pub type TvrefLongF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - JEDEC TCKFSPE, the valid clock requirement after entering SDP change."]
    #[inline(always)]
    pub fn tckfspe_f0(&self) -> TckfspeF0R {
        TckfspeF0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - JEDEC TCKFSPX, the valid clock requirement before 1st valid command after FSP change."]
    #[inline(always)]
    pub fn tckfspx_f0(&self) -> TckfspxF0R {
        TckfspxF0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - JEDEC TVREF, design will always use the long value."]
    #[inline(always)]
    pub fn tvref_long_f0(&self) -> TvrefLongF0R {
        TvrefLongF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - JEDEC TCKFSPE, the valid clock requirement after entering SDP change."]
    #[inline(always)]
    #[must_use]
    pub fn tckfspe_f0(&mut self) -> TckfspeF0W<DenaliCtl124Spec> {
        TckfspeF0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - JEDEC TCKFSPX, the valid clock requirement before 1st valid command after FSP change."]
    #[inline(always)]
    #[must_use]
    pub fn tckfspx_f0(&mut self) -> TckfspxF0W<DenaliCtl124Spec> {
        TckfspxF0W::new(self, 8)
    }
    #[doc = "Bits 16:31 - JEDEC TVREF, design will always use the long value."]
    #[inline(always)]
    #[must_use]
    pub fn tvref_long_f0(&mut self) -> TvrefLongF0W<DenaliCtl124Spec> {
        TvrefLongF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_124::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_124::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl124Spec;
impl crate::RegisterSpec for DenaliCtl124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_124::R`](R) reader structure"]
impl crate::Readable for DenaliCtl124Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_124::W`](W) writer structure"]
impl crate::Writable for DenaliCtl124Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_124 to value 0"]
impl crate::Resettable for DenaliCtl124Spec {
    const RESET_VALUE: u32 = 0;
}
