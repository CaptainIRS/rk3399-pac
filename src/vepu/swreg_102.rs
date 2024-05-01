#[doc = "Register `SWREG_102` reader"]
pub type R = crate::R<Swreg102Spec>;
#[doc = "Register `SWREG_102` writer"]
pub type W = crate::W<Swreg102Spec>;
#[doc = "Field `MVC_INTER_VIEW_FLAG` reader - the inter-view prediction of picture\n\nMVC inter_view_flag."]
pub type MvcInterViewFlagR = crate::BitReader;
#[doc = "Field `MVC_INTER_VIEW_FLAG` writer - the inter-view prediction of picture\n\nMVC inter_view_flag."]
pub type MvcInterViewFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVC_TEMPORAL_ID` reader - MVC temporal_id"]
pub type MvcTemporalIdR = crate::FieldReader;
#[doc = "Field `MVC_TEMPORAL_ID` writer - MVC temporal_id"]
pub type MvcTemporalIdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MVC_PRIORITY_ID` reader - MVC priority_id"]
pub type MvcPriorityIdR = crate::FieldReader;
#[doc = "Field `MVC_PRIORITY_ID` writer - MVC priority_id"]
pub type MvcPriorityIdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MVC_ANCHOR_PIC_FLAG` reader - to specifie picture is one part of anchor access unit"]
pub type MvcAnchorPicFlagR = crate::BitReader;
#[doc = "Field `MVC_ANCHOR_PIC_FLAG` writer - to specifie picture is one part of anchor access unit"]
pub type MvcAnchorPicFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVC_VIEW_ID` reader - MVC view_id"]
pub type MvcViewIdR = crate::FieldReader;
#[doc = "Field `MVC_VIEW_ID` writer - MVC view_id"]
pub type MvcViewIdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MV_PLY_4X4` reader - 4x4 Mv Penalty"]
pub type MvPly4x4R = crate::FieldReader<u16>;
#[doc = "Field `MV_PLY_4X4` writer - 4x4 Mv Penalty"]
pub type MvPly4x4W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MV_FAVOR_16X16` reader - Zero 16x16 MV favor\n\nvalue = (real value)/2."]
pub type MvFavor16x16R = crate::FieldReader;
#[doc = "Field `MV_FAVOR_16X16` writer - Zero 16x16 MV favor\n\nvalue = (real value)/2."]
pub type MvFavor16x16W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - the inter-view prediction of picture\n\nMVC inter_view_flag."]
    #[inline(always)]
    pub fn mvc_inter_view_flag(&self) -> MvcInterViewFlagR {
        MvcInterViewFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - MVC temporal_id"]
    #[inline(always)]
    pub fn mvc_temporal_id(&self) -> MvcTemporalIdR {
        MvcTemporalIdR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - MVC priority_id"]
    #[inline(always)]
    pub fn mvc_priority_id(&self) -> MvcPriorityIdR {
        MvcPriorityIdR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - to specifie picture is one part of anchor access unit"]
    #[inline(always)]
    pub fn mvc_anchor_pic_flag(&self) -> MvcAnchorPicFlagR {
        MvcAnchorPicFlagR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - MVC view_id"]
    #[inline(always)]
    pub fn mvc_view_id(&self) -> MvcViewIdR {
        MvcViewIdR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:19 - 4x4 Mv Penalty"]
    #[inline(always)]
    pub fn mv_ply_4x4(&self) -> MvPly4x4R {
        MvPly4x4R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:23 - Zero 16x16 MV favor\n\nvalue = (real value)/2."]
    #[inline(always)]
    pub fn mv_favor_16x16(&self) -> MvFavor16x16R {
        MvFavor16x16R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - the inter-view prediction of picture\n\nMVC inter_view_flag."]
    #[inline(always)]
    #[must_use]
    pub fn mvc_inter_view_flag(&mut self) -> MvcInterViewFlagW<Swreg102Spec> {
        MvcInterViewFlagW::new(self, 0)
    }
    #[doc = "Bits 1:3 - MVC temporal_id"]
    #[inline(always)]
    #[must_use]
    pub fn mvc_temporal_id(&mut self) -> MvcTemporalIdW<Swreg102Spec> {
        MvcTemporalIdW::new(self, 1)
    }
    #[doc = "Bits 4:6 - MVC priority_id"]
    #[inline(always)]
    #[must_use]
    pub fn mvc_priority_id(&mut self) -> MvcPriorityIdW<Swreg102Spec> {
        MvcPriorityIdW::new(self, 4)
    }
    #[doc = "Bit 7 - to specifie picture is one part of anchor access unit"]
    #[inline(always)]
    #[must_use]
    pub fn mvc_anchor_pic_flag(&mut self) -> MvcAnchorPicFlagW<Swreg102Spec> {
        MvcAnchorPicFlagW::new(self, 7)
    }
    #[doc = "Bits 8:10 - MVC view_id"]
    #[inline(always)]
    #[must_use]
    pub fn mvc_view_id(&mut self) -> MvcViewIdW<Swreg102Spec> {
        MvcViewIdW::new(self, 8)
    }
    #[doc = "Bits 11:19 - 4x4 Mv Penalty"]
    #[inline(always)]
    #[must_use]
    pub fn mv_ply_4x4(&mut self) -> MvPly4x4W<Swreg102Spec> {
        MvPly4x4W::new(self, 11)
    }
    #[doc = "Bits 20:23 - Zero 16x16 MV favor\n\nvalue = (real value)/2."]
    #[inline(always)]
    #[must_use]
    pub fn mv_favor_16x16(&mut self) -> MvFavor16x16W<Swreg102Spec> {
        MvFavor16x16W::new(self, 20)
    }
}
#[doc = "mvc related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_102::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_102::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg102Spec;
impl crate::RegisterSpec for Swreg102Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_102::R`](R) reader structure"]
impl crate::Readable for Swreg102Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_102::W`](W) writer structure"]
impl crate::Writable for Swreg102Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_102 to value 0"]
impl crate::Resettable for Swreg102Spec {
    const RESET_VALUE: u32 = 0;
}
