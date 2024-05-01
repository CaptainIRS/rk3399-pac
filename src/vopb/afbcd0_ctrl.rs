#[doc = "Register `AFBCD0_CTRL` reader"]
pub type R = crate::R<Afbcd0CtrlSpec>;
#[doc = "Register `AFBCD0_CTRL` writer"]
pub type W = crate::W<Afbcd0CtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VopbFbdcEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<VopbFbdcEn> for bool {
    #[inline(always)]
    fn from(variant: VopbFbdcEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOPB_FBDC_EN` reader - "]
pub type VopbFbdcEnR = crate::BitReader<VopbFbdcEn>;
impl VopbFbdcEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopbFbdcEn {
        match self.bits {
            false => VopbFbdcEn::B0,
            true => VopbFbdcEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VopbFbdcEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VopbFbdcEn::B1
    }
}
#[doc = "Field `VOPB_FBDC_EN` writer - "]
pub type VopbFbdcEnW<'a, REG> = crate::BitWriter<'a, REG, VopbFbdcEn>;
impl<'a, REG> VopbFbdcEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VopbFbdcEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VopbFbdcEn::B1)
    }
}
#[doc = "Field `VOPB_FBDC_WIN_SEL` reader - seleclt fbdc to layer 2'bxx"]
pub type VopbFbdcWinSelR = crate::FieldReader;
#[doc = "Field `VOPB_FBDC_WIN_SEL` writer - seleclt fbdc to layer 2'bxx"]
pub type VopbFbdcWinSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FBDC_RSTN` reader - fbdc_rstn"]
pub type FbdcRstnR = crate::BitReader;
#[doc = "Field `FBDC_RSTN` writer - fbdc_rstn"]
pub type FbdcRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOPB_FBDC_AXI_MAX_OUTSTANDING_NUM` reader - VOPB_fbdc_axi_max_outstanding_num"]
pub type VopbFbdcAxiMaxOutstandingNumR = crate::FieldReader;
#[doc = "Field `VOPB_FBDC_AXI_MAX_OUTSTANDING_NUM` writer - VOPB_fbdc_axi_max_outstanding_num"]
pub type VopbFbdcAxiMaxOutstandingNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VOPB_FBDC_AXI_MAX_OUTSTANDING_EN` reader - VOPB_fbdc_axi_max_outstanding_en"]
pub type VopbFbdcAxiMaxOutstandingEnR = crate::BitReader;
#[doc = "Field `VOPB_FBDC_AXI_MAX_OUTSTANDING_EN` writer - VOPB_fbdc_axi_max_outstanding_en"]
pub type VopbFbdcAxiMaxOutstandingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBDC_RID` reader - fbdc_rid"]
pub type FbdcRidR = crate::FieldReader;
#[doc = "Field `FBDC_RID` writer - fbdc_rid"]
pub type FbdcRidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFBCD_HREG_PIXEL_PACKING_FMT` reader - afbcd_hreg_pixel_packing_fmt"]
pub type AfbcdHregPixelPackingFmtR = crate::FieldReader;
#[doc = "Field `AFBCD_HREG_PIXEL_PACKING_FMT` writer - afbcd_hreg_pixel_packing_fmt"]
pub type AfbcdHregPixelPackingFmtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AFBCD_HREG_BLOCK_SPLIT` reader - afbcd_hreg_block_split"]
pub type AfbcdHregBlockSplitR = crate::BitReader;
#[doc = "Field `AFBCD_HREG_BLOCK_SPLIT` writer - afbcd_hreg_block_split"]
pub type AfbcdHregBlockSplitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vopb_fbdc_en(&self) -> VopbFbdcEnR {
        VopbFbdcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - seleclt fbdc to layer 2'bxx"]
    #[inline(always)]
    pub fn vopb_fbdc_win_sel(&self) -> VopbFbdcWinSelR {
        VopbFbdcWinSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - fbdc_rstn"]
    #[inline(always)]
    pub fn fbdc_rstn(&self) -> FbdcRstnR {
        FbdcRstnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - VOPB_fbdc_axi_max_outstanding_num"]
    #[inline(always)]
    pub fn vopb_fbdc_axi_max_outstanding_num(&self) -> VopbFbdcAxiMaxOutstandingNumR {
        VopbFbdcAxiMaxOutstandingNumR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - VOPB_fbdc_axi_max_outstanding_en"]
    #[inline(always)]
    pub fn vopb_fbdc_axi_max_outstanding_en(&self) -> VopbFbdcAxiMaxOutstandingEnR {
        VopbFbdcAxiMaxOutstandingEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:15 - fbdc_rid"]
    #[inline(always)]
    pub fn fbdc_rid(&self) -> FbdcRidR {
        FbdcRidR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - afbcd_hreg_pixel_packing_fmt"]
    #[inline(always)]
    pub fn afbcd_hreg_pixel_packing_fmt(&self) -> AfbcdHregPixelPackingFmtR {
        AfbcdHregPixelPackingFmtR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - afbcd_hreg_block_split"]
    #[inline(always)]
    pub fn afbcd_hreg_block_split(&self) -> AfbcdHregBlockSplitR {
        AfbcdHregBlockSplitR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_fbdc_en(&mut self) -> VopbFbdcEnW<Afbcd0CtrlSpec> {
        VopbFbdcEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - seleclt fbdc to layer 2'bxx"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_fbdc_win_sel(&mut self) -> VopbFbdcWinSelW<Afbcd0CtrlSpec> {
        VopbFbdcWinSelW::new(self, 1)
    }
    #[doc = "Bit 3 - fbdc_rstn"]
    #[inline(always)]
    #[must_use]
    pub fn fbdc_rstn(&mut self) -> FbdcRstnW<Afbcd0CtrlSpec> {
        FbdcRstnW::new(self, 3)
    }
    #[doc = "Bits 4:8 - VOPB_fbdc_axi_max_outstanding_num"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_fbdc_axi_max_outstanding_num(
        &mut self,
    ) -> VopbFbdcAxiMaxOutstandingNumW<Afbcd0CtrlSpec> {
        VopbFbdcAxiMaxOutstandingNumW::new(self, 4)
    }
    #[doc = "Bit 9 - VOPB_fbdc_axi_max_outstanding_en"]
    #[inline(always)]
    #[must_use]
    pub fn vopb_fbdc_axi_max_outstanding_en(
        &mut self,
    ) -> VopbFbdcAxiMaxOutstandingEnW<Afbcd0CtrlSpec> {
        VopbFbdcAxiMaxOutstandingEnW::new(self, 9)
    }
    #[doc = "Bits 12:15 - fbdc_rid"]
    #[inline(always)]
    #[must_use]
    pub fn fbdc_rid(&mut self) -> FbdcRidW<Afbcd0CtrlSpec> {
        FbdcRidW::new(self, 12)
    }
    #[doc = "Bits 16:20 - afbcd_hreg_pixel_packing_fmt"]
    #[inline(always)]
    #[must_use]
    pub fn afbcd_hreg_pixel_packing_fmt(&mut self) -> AfbcdHregPixelPackingFmtW<Afbcd0CtrlSpec> {
        AfbcdHregPixelPackingFmtW::new(self, 16)
    }
    #[doc = "Bit 21 - afbcd_hreg_block_split"]
    #[inline(always)]
    #[must_use]
    pub fn afbcd_hreg_block_split(&mut self) -> AfbcdHregBlockSplitW<Afbcd0CtrlSpec> {
        AfbcdHregBlockSplitW::new(self, 21)
    }
}
#[doc = "AFBCD0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afbcd0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afbcd0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afbcd0CtrlSpec;
impl crate::RegisterSpec for Afbcd0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afbcd0_ctrl::R`](R) reader structure"]
impl crate::Readable for Afbcd0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afbcd0_ctrl::W`](W) writer structure"]
impl crate::Writable for Afbcd0CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFBCD0_CTRL to value 0"]
impl crate::Resettable for Afbcd0CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
