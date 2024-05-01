#[doc = "Register `SWREG_76_REUSE` reader"]
pub type R = crate::R<Swreg76ReuseSpec>;
#[doc = "Register `SWREG_76_REUSE` writer"]
pub type W = crate::W<Swreg76ReuseSpec>;
#[doc = "Field `CONSTR_INTRA_PRED` reader - constrained intra prediction\n\nconstrained intra prediction"]
pub type ConstrIntraPredR = crate::BitReader;
#[doc = "Field `CONSTR_INTRA_PRED` writer - constrained intra prediction\n\nconstrained intra prediction"]
pub type ConstrIntraPredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDR_PICID` reader - IDR pic ID\n\nIDR pic ID"]
pub type IdrPicidR = crate::FieldReader;
#[doc = "Field `IDR_PICID` writer - IDR pic ID\n\nIDR pic ID"]
pub type IdrPicidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_QPASS` reader - jpeg enc quant bypass"]
pub type SwQpassR = crate::BitReader;
#[doc = "Field `SW_QPASS` writer - jpeg enc quant bypass"]
pub type SwQpassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QP_OFFSET_CH` reader - the qp index offset for chroma qp used in h264\n\nsigned register\n\nrange : -12~12"]
pub type QpOffsetChR = crate::FieldReader;
#[doc = "Field `QP_OFFSET_CH` writer - the qp index offset for chroma qp used in h264\n\nsigned register\n\nrange : -12~12"]
pub type QpOffsetChW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SLICE_FLT_BETA` reader - the filter beta offset for h264 slice\n\nconfig value = (real value)/2\n\nsigned register\n\nrange : -6 ~6"]
pub type SliceFltBetaR = crate::FieldReader;
#[doc = "Field `SLICE_FLT_BETA` writer - the filter beta offset for h264 slice\n\nconfig value = (real value)/2\n\nsigned register\n\nrange : -6 ~6"]
pub type SliceFltBetaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLICE_FLT_ALPHA` reader - the offset of slice filter alpha c0 used in h264\n\noffset div2\n\nrange : -6~6"]
pub type SliceFltAlphaR = crate::FieldReader;
#[doc = "Field `SLICE_FLT_ALPHA` writer - the offset of slice filter alpha c0 used in h264\n\noffset div2\n\nrange : -6~6"]
pub type SliceFltAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPS_INIT_QP` reader - pps init qp in picture used in h264\n\npps init qp in picture used in h264\n\nrange : 0~51"]
pub type PpsInitQpR = crate::FieldReader;
#[doc = "Field `PPS_INIT_QP` writer - pps init qp in picture used in h264\n\npps init qp in picture used in h264\n\nrange : 0~51"]
pub type PpsInitQpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - constrained intra prediction\n\nconstrained intra prediction"]
    #[inline(always)]
    pub fn constr_intra_pred(&self) -> ConstrIntraPredR {
        ConstrIntraPredR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - IDR pic ID\n\nIDR pic ID"]
    #[inline(always)]
    pub fn idr_picid(&self) -> IdrPicidR {
        IdrPicidR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - jpeg enc quant bypass"]
    #[inline(always)]
    pub fn sw_qpass(&self) -> SwQpassR {
        SwQpassR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 13:17 - the qp index offset for chroma qp used in h264\n\nsigned register\n\nrange : -12~12"]
    #[inline(always)]
    pub fn qp_offset_ch(&self) -> QpOffsetChR {
        QpOffsetChR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:21 - the filter beta offset for h264 slice\n\nconfig value = (real value)/2\n\nsigned register\n\nrange : -6 ~6"]
    #[inline(always)]
    pub fn slice_flt_beta(&self) -> SliceFltBetaR {
        SliceFltBetaR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - the offset of slice filter alpha c0 used in h264\n\noffset div2\n\nrange : -6~6"]
    #[inline(always)]
    pub fn slice_flt_alpha(&self) -> SliceFltAlphaR {
        SliceFltAlphaR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:31 - pps init qp in picture used in h264\n\npps init qp in picture used in h264\n\nrange : 0~51"]
    #[inline(always)]
    pub fn pps_init_qp(&self) -> PpsInitQpR {
        PpsInitQpR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - constrained intra prediction\n\nconstrained intra prediction"]
    #[inline(always)]
    #[must_use]
    pub fn constr_intra_pred(&mut self) -> ConstrIntraPredW<Swreg76ReuseSpec> {
        ConstrIntraPredW::new(self, 0)
    }
    #[doc = "Bits 1:4 - IDR pic ID\n\nIDR pic ID"]
    #[inline(always)]
    #[must_use]
    pub fn idr_picid(&mut self) -> IdrPicidW<Swreg76ReuseSpec> {
        IdrPicidW::new(self, 1)
    }
    #[doc = "Bit 8 - jpeg enc quant bypass"]
    #[inline(always)]
    #[must_use]
    pub fn sw_qpass(&mut self) -> SwQpassW<Swreg76ReuseSpec> {
        SwQpassW::new(self, 8)
    }
    #[doc = "Bits 13:17 - the qp index offset for chroma qp used in h264\n\nsigned register\n\nrange : -12~12"]
    #[inline(always)]
    #[must_use]
    pub fn qp_offset_ch(&mut self) -> QpOffsetChW<Swreg76ReuseSpec> {
        QpOffsetChW::new(self, 13)
    }
    #[doc = "Bits 18:21 - the filter beta offset for h264 slice\n\nconfig value = (real value)/2\n\nsigned register\n\nrange : -6 ~6"]
    #[inline(always)]
    #[must_use]
    pub fn slice_flt_beta(&mut self) -> SliceFltBetaW<Swreg76ReuseSpec> {
        SliceFltBetaW::new(self, 18)
    }
    #[doc = "Bits 22:25 - the offset of slice filter alpha c0 used in h264\n\noffset div2\n\nrange : -6~6"]
    #[inline(always)]
    #[must_use]
    pub fn slice_flt_alpha(&mut self) -> SliceFltAlphaW<Swreg76ReuseSpec> {
        SliceFltAlphaW::new(self, 22)
    }
    #[doc = "Bits 26:31 - pps init qp in picture used in h264\n\npps init qp in picture used in h264\n\nrange : 0~51"]
    #[inline(always)]
    #[must_use]
    pub fn pps_init_qp(&mut self) -> PpsInitQpW<Swreg76ReuseSpec> {
        PpsInitQpW::new(self, 26)
    }
}
#[doc = "encoder control regsiter 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_76_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_76_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg76ReuseSpec;
impl crate::RegisterSpec for Swreg76ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_76_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg76ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_76_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg76ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_76_REUSE to value 0"]
impl crate::Resettable for Swreg76ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
