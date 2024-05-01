#[doc = "Register `SWREG115` reader"]
pub type R = crate::R<Swreg115Spec>;
#[doc = "Register `SWREG115` writer"]
pub type W = crate::W<Swreg115Spec>;
#[doc = "Field `H264_FIELDPIC_FLAG_EXIST` reader - Flag for streamd that field_pic_flag exists in stream\n\nFlag for streamd that field_pic_flag exists in stream"]
pub type H264FieldpicFlagExistR = crate::BitReader;
#[doc = "Field `H264_FIELDPIC_FLAG_EXIST` writer - Flag for streamd that field_pic_flag exists in stream\n\nFlag for streamd that field_pic_flag exists in stream"]
pub type H264FieldpicFlagExistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "scaling matrix enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264SclMatrixEn {
    #[doc = "0: normal transform"]
    B0 = 0,
    #[doc = "1: use scaling matrix for transform"]
    B1 = 1,
}
impl From<H264SclMatrixEn> for bool {
    #[inline(always)]
    fn from(variant: H264SclMatrixEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_SCL_MATRIX_EN` reader - scaling matrix enable"]
pub type H264SclMatrixEnR = crate::BitReader<H264SclMatrixEn>;
impl H264SclMatrixEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264SclMatrixEn {
        match self.bits {
            false => H264SclMatrixEn::B0,
            true => H264SclMatrixEn::B1,
        }
    }
    #[doc = "normal transform"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264SclMatrixEn::B0
    }
    #[doc = "use scaling matrix for transform"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264SclMatrixEn::B1
    }
}
#[doc = "Field `H264_SCL_MATRIX_EN` writer - scaling matrix enable"]
pub type H264SclMatrixEnW<'a, REG> = crate::BitWriter<'a, REG, H264SclMatrixEn>;
impl<'a, REG> H264SclMatrixEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal transform"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264SclMatrixEn::B0)
    }
    #[doc = "use scaling matrix for transform"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264SclMatrixEn::B1)
    }
}
#[doc = "Field `H264_TRANF_FLAG_EN_8X8` reader - 8x8 transform flag enable for stream decoding"]
pub type H264TranfFlagEn8x8R = crate::BitReader;
#[doc = "Field `H264_TRANF_FLAG_EN_8X8` writer - 8x8 transform flag enable for stream decoding"]
pub type H264TranfFlagEn8x8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "if intra prediction uses only neighbouring intra macroblocks\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264NimbIntraEn {
    #[doc = "0: neighbouring inter macroblocks are used in intra prediction process"]
    B0 = 0,
    #[doc = "1: neighbouring intra macroblocks are used"]
    B1 = 1,
}
impl From<H264NimbIntraEn> for bool {
    #[inline(always)]
    fn from(variant: H264NimbIntraEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_NIMB_INTRA_EN` reader - if intra prediction uses only neighbouring intra macroblocks"]
pub type H264NimbIntraEnR = crate::BitReader<H264NimbIntraEn>;
impl H264NimbIntraEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264NimbIntraEn {
        match self.bits {
            false => H264NimbIntraEn::B0,
            true => H264NimbIntraEn::B1,
        }
    }
    #[doc = "neighbouring inter macroblocks are used in intra prediction process"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264NimbIntraEn::B0
    }
    #[doc = "neighbouring intra macroblocks are used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264NimbIntraEn::B1
    }
}
#[doc = "Field `H264_NIMB_INTRA_EN` writer - if intra prediction uses only neighbouring intra macroblocks"]
pub type H264NimbIntraEnW<'a, REG> = crate::BitWriter<'a, REG, H264NimbIntraEn>;
impl<'a, REG> H264NimbIntraEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "neighbouring inter macroblocks are used in intra prediction process"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264NimbIntraEn::B0)
    }
    #[doc = "neighbouring intra macroblocks are used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264NimbIntraEn::B1)
    }
}
#[doc = "Field `H264_PSLICE_WP_EN` reader - enable flag of Weighted prediction for P slices"]
pub type H264PsliceWpEnR = crate::BitReader;
#[doc = "Field `H264_PSLICE_WP_EN` writer - enable flag of Weighted prediction for P slices"]
pub type H264PsliceWpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_CABAC_EN` reader - enable for cabac"]
pub type H264CabacEnR = crate::BitReader;
#[doc = "Field `H264_CABAC_EN` writer - enable for cabac"]
pub type H264CabacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "monochromatic enable\n\nsampling format ,\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264MonochrEn {
    #[doc = "0: 4:2:0"]
    B0 = 0,
    #[doc = "1: 4:0:0"]
    B1 = 1,
}
impl From<H264MonochrEn> for bool {
    #[inline(always)]
    fn from(variant: H264MonochrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_MONOCHR_EN` reader - monochromatic enable\n\nsampling format ,"]
pub type H264MonochrEnR = crate::BitReader<H264MonochrEn>;
impl H264MonochrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264MonochrEn {
        match self.bits {
            false => H264MonochrEn::B0,
            true => H264MonochrEn::B1,
        }
    }
    #[doc = "4:2:0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264MonochrEn::B0
    }
    #[doc = "4:0:0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264MonochrEn::B1
    }
}
#[doc = "Field `H264_MONOCHR_EN` writer - monochromatic enable\n\nsampling format ,"]
pub type H264MonochrEnW<'a, REG> = crate::BitWriter<'a, REG, H264MonochrEn>;
impl<'a, REG> H264MonochrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4:2:0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264MonochrEn::B0)
    }
    #[doc = "4:0:0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264MonochrEn::B1)
    }
}
#[doc = "Field `H264_DLMV_METHOD_EN` reader - the method to use to derive luma motion vectors\n\nwith B_skip, B_Direct_16x16 and B_direct_8x8_inference_flag"]
pub type H264DlmvMethodEnR = crate::BitReader;
#[doc = "Field `H264_DLMV_METHOD_EN` writer - the method to use to derive luma motion vectors\n\nwith B_skip, B_Direct_16x16 and B_direct_8x8_inference_flag"]
pub type H264DlmvMethodEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_IDR_PIC_FLAG` reader - instantaneous decoding refresh picture flag"]
pub type H264IdrPicFlagR = crate::BitReader;
#[doc = "Field `H264_IDR_PIC_FLAG` writer - instantaneous decoding refresh picture flag"]
pub type H264IdrPicFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flag for streamd that field_pic_flag exists in stream\n\nFlag for streamd that field_pic_flag exists in stream"]
    #[inline(always)]
    pub fn h264_fieldpic_flag_exist(&self) -> H264FieldpicFlagExistR {
        H264FieldpicFlagExistR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - scaling matrix enable"]
    #[inline(always)]
    pub fn h264_scl_matrix_en(&self) -> H264SclMatrixEnR {
        H264SclMatrixEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8x8 transform flag enable for stream decoding"]
    #[inline(always)]
    pub fn h264_tranf_flag_en_8x8(&self) -> H264TranfFlagEn8x8R {
        H264TranfFlagEn8x8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - if intra prediction uses only neighbouring intra macroblocks"]
    #[inline(always)]
    pub fn h264_nimb_intra_en(&self) -> H264NimbIntraEnR {
        H264NimbIntraEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable flag of Weighted prediction for P slices"]
    #[inline(always)]
    pub fn h264_pslice_wp_en(&self) -> H264PsliceWpEnR {
        H264PsliceWpEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable for cabac"]
    #[inline(always)]
    pub fn h264_cabac_en(&self) -> H264CabacEnR {
        H264CabacEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - monochromatic enable\n\nsampling format ,"]
    #[inline(always)]
    pub fn h264_monochr_en(&self) -> H264MonochrEnR {
        H264MonochrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the method to use to derive luma motion vectors\n\nwith B_skip, B_Direct_16x16 and B_direct_8x8_inference_flag"]
    #[inline(always)]
    pub fn h264_dlmv_method_en(&self) -> H264DlmvMethodEnR {
        H264DlmvMethodEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - instantaneous decoding refresh picture flag"]
    #[inline(always)]
    pub fn h264_idr_pic_flag(&self) -> H264IdrPicFlagR {
        H264IdrPicFlagR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flag for streamd that field_pic_flag exists in stream\n\nFlag for streamd that field_pic_flag exists in stream"]
    #[inline(always)]
    #[must_use]
    pub fn h264_fieldpic_flag_exist(&mut self) -> H264FieldpicFlagExistW<Swreg115Spec> {
        H264FieldpicFlagExistW::new(self, 0)
    }
    #[doc = "Bit 1 - scaling matrix enable"]
    #[inline(always)]
    #[must_use]
    pub fn h264_scl_matrix_en(&mut self) -> H264SclMatrixEnW<Swreg115Spec> {
        H264SclMatrixEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 8x8 transform flag enable for stream decoding"]
    #[inline(always)]
    #[must_use]
    pub fn h264_tranf_flag_en_8x8(&mut self) -> H264TranfFlagEn8x8W<Swreg115Spec> {
        H264TranfFlagEn8x8W::new(self, 2)
    }
    #[doc = "Bit 3 - if intra prediction uses only neighbouring intra macroblocks"]
    #[inline(always)]
    #[must_use]
    pub fn h264_nimb_intra_en(&mut self) -> H264NimbIntraEnW<Swreg115Spec> {
        H264NimbIntraEnW::new(self, 3)
    }
    #[doc = "Bit 4 - enable flag of Weighted prediction for P slices"]
    #[inline(always)]
    #[must_use]
    pub fn h264_pslice_wp_en(&mut self) -> H264PsliceWpEnW<Swreg115Spec> {
        H264PsliceWpEnW::new(self, 4)
    }
    #[doc = "Bit 5 - enable for cabac"]
    #[inline(always)]
    #[must_use]
    pub fn h264_cabac_en(&mut self) -> H264CabacEnW<Swreg115Spec> {
        H264CabacEnW::new(self, 5)
    }
    #[doc = "Bit 6 - monochromatic enable\n\nsampling format ,"]
    #[inline(always)]
    #[must_use]
    pub fn h264_monochr_en(&mut self) -> H264MonochrEnW<Swreg115Spec> {
        H264MonochrEnW::new(self, 6)
    }
    #[doc = "Bit 7 - the method to use to derive luma motion vectors\n\nwith B_skip, B_Direct_16x16 and B_direct_8x8_inference_flag"]
    #[inline(always)]
    #[must_use]
    pub fn h264_dlmv_method_en(&mut self) -> H264DlmvMethodEnW<Swreg115Spec> {
        H264DlmvMethodEnW::new(self, 7)
    }
    #[doc = "Bit 8 - instantaneous decoding refresh picture flag"]
    #[inline(always)]
    #[must_use]
    pub fn h264_idr_pic_flag(&mut self) -> H264IdrPicFlagW<Swreg115Spec> {
        H264IdrPicFlagW::new(self, 8)
    }
}
#[doc = "enable flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg115::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg115::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg115Spec;
impl crate::RegisterSpec for Swreg115Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg115::R`](R) reader structure"]
impl crate::Readable for Swreg115Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg115::W`](W) writer structure"]
impl crate::Writable for Swreg115Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG115 to value 0"]
impl crate::Resettable for Swreg115Spec {
    const RESET_VALUE: u32 = 0;
}
