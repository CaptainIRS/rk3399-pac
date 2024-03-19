#[doc = "Register `DDR_PI_REG_186` reader"]
pub type R = crate::R<DdrPiReg186Spec>;
#[doc = "Register `DDR_PI_REG_186` writer"]
pub type W = crate::W<DdrPiReg186Spec>;
#[doc = "Field `PI_BSTLEN` reader - Indicates encoded burst length that is sent to DRAMs during initialization. Set to 1 for BL2, set to 2 for BL4, or set to 3 for BL8."]
pub type PiBstlenR = crate::FieldReader;
#[doc = "Field `PI_BSTLEN` writer - Indicates encoded burst length that is sent to DRAMs during initialization. Set to 1 for BL2, set to 2 for BL4, or set to 3 for BL8."]
pub type PiBstlenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_LONG_COUNT_MASK` reader - Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 (1024 cycles), 0x10 (512 clocks), 0x18 (256 clocks), 0x1C (128 clocks), 0x1E (64 clocks), and 0x1F (32 clocks)."]
pub type PiLongCountMaskR = crate::FieldReader;
#[doc = "Field `PI_LONG_COUNT_MASK` writer - Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 (1024 cycles), 0x10 (512 clocks), 0x18 (256 clocks), 0x1C (128 clocks), 0x1E (64 clocks), and 0x1F (32 clocks)."]
pub type PiLongCountMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_CTRLUPD_REQ_PER_AREF_EN` reader - Enables an automatic PI-initiated update (dfi_ctrlupd_req) after every refresh. Set to 1 to enable."]
pub type PiCtrlupdReqPerArefEnR = crate::BitReader;
#[doc = "Field `PI_CTRLUPD_REQ_PER_AREF_EN` writer - Enables an automatic PI-initiated update (dfi_ctrlupd_req) after every refresh. Set to 1 to enable."]
pub type PiCtrlupdReqPerArefEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:12 - Indicates encoded burst length that is sent to DRAMs during initialization. Set to 1 for BL2, set to 2 for BL4, or set to 3 for BL8."]
    #[inline(always)]
    pub fn pi_bstlen(&self) -> PiBstlenR {
        PiBstlenR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 (1024 cycles), 0x10 (512 clocks), 0x18 (256 clocks), 0x1C (128 clocks), 0x1E (64 clocks), and 0x1F (32 clocks)."]
    #[inline(always)]
    pub fn pi_long_count_mask(&self) -> PiLongCountMaskR {
        PiLongCountMaskR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Enables an automatic PI-initiated update (dfi_ctrlupd_req) after every refresh. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_ctrlupd_req_per_aref_en(&self) -> PiCtrlupdReqPerArefEnR {
        PiCtrlupdReqPerArefEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - Indicates encoded burst length that is sent to DRAMs during initialization. Set to 1 for BL2, set to 2 for BL4, or set to 3 for BL8."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bstlen(&mut self) -> PiBstlenW<DdrPiReg186Spec> {
        PiBstlenW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 (1024 cycles), 0x10 (512 clocks), 0x18 (256 clocks), 0x1C (128 clocks), 0x1E (64 clocks), and 0x1F (32 clocks)."]
    #[inline(always)]
    #[must_use]
    pub fn pi_long_count_mask(&mut self) -> PiLongCountMaskW<DdrPiReg186Spec> {
        PiLongCountMaskW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables an automatic PI-initiated update (dfi_ctrlupd_req) after every refresh. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ctrlupd_req_per_aref_en(&mut self) -> PiCtrlupdReqPerArefEnW<DdrPiReg186Spec> {
        PiCtrlupdReqPerArefEnW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 186\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_186::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_186::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg186Spec;
impl crate::RegisterSpec for DdrPiReg186Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_186::R`](R) reader structure"]
impl crate::Readable for DdrPiReg186Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_186::W`](W) writer structure"]
impl crate::Writable for DdrPiReg186Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_186 to value 0"]
impl crate::Resettable for DdrPiReg186Spec {
    const RESET_VALUE: u32 = 0;
}
