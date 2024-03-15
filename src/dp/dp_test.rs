#[doc = "Register `DP_TEST` reader"]
pub type R = crate::R<DpTestSpec>;
#[doc = "Register `DP_TEST` writer"]
pub type W = crate::W<DpTestSpec>;
#[doc = "Field `DP_TEST_BITS_1` reader - When &lt;7:6> ==10, test analog blocks: 000-- disable analog test 001: enable ch0 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 010: enable ch1 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 011: enable ch2 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 100: enable ch3 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 101: enable pll analog test mux &lt;1:0>(00/01/10/11)--->(vdd10_cln,v1p45_v2i, avdd18,vco_ctrl) 110: enable charge pump regulator analog test mux &lt;1:0> (00/01/10/11)---->(v1v_regu, vregu_out,v0.5_ref,null) 111: test band gap output When&lt;7:6>==11, test digital blocks: 000-- disable digital test 001: enable pll digital test mux &lt;1:0>--(00/01/10/11)--->pll_ref,vco_fb, vss,vss. 010: test charge pump regulator OSC clock."]
pub type DpTestBits1R = crate::FieldReader;
#[doc = "Field `DP_TEST_BITS_1` writer - When &lt;7:6> ==10, test analog blocks: 000-- disable analog test 001: enable ch0 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 010: enable ch1 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 011: enable ch2 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 100: enable ch3 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 101: enable pll analog test mux &lt;1:0>(00/01/10/11)--->(vdd10_cln,v1p45_v2i, avdd18,vco_ctrl) 110: enable charge pump regulator analog test mux &lt;1:0> (00/01/10/11)---->(v1v_regu, vregu_out,v0.5_ref,null) 111: test band gap output When&lt;7:6>==11, test digital blocks: 000-- disable digital test 001: enable pll digital test mux &lt;1:0>--(00/01/10/11)--->pll_ref,vco_fb, vss,vss. 010: test charge pump regulator OSC clock."]
pub type DpTestBits1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DP_TEST_MODE` reader - 00&amp;01: test disables dc_tp/atesto/dtesto output hiz. 10: atesto test enable 11: dtesto test enable"]
pub type DpTestModeR = crate::FieldReader;
#[doc = "Field `DP_TEST_MODE` writer - 00&amp;01: test disables dc_tp/atesto/dtesto output hiz. 10: atesto test enable 11: dtesto test enable"]
pub type DpTestModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 3:5 - When &lt;7:6> ==10, test analog blocks: 000-- disable analog test 001: enable ch0 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 010: enable ch1 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 011: enable ch2 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 100: enable ch3 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 101: enable pll analog test mux &lt;1:0>(00/01/10/11)--->(vdd10_cln,v1p45_v2i, avdd18,vco_ctrl) 110: enable charge pump regulator analog test mux &lt;1:0> (00/01/10/11)---->(v1v_regu, vregu_out,v0.5_ref,null) 111: test band gap output When&lt;7:6>==11, test digital blocks: 000-- disable digital test 001: enable pll digital test mux &lt;1:0>--(00/01/10/11)--->pll_ref,vco_fb, vss,vss. 010: test charge pump regulator OSC clock."]
    #[inline(always)]
    pub fn dp_test_bits_1(&self) -> DpTestBits1R {
        DpTestBits1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - 00&amp;01: test disables dc_tp/atesto/dtesto output hiz. 10: atesto test enable 11: dtesto test enable"]
    #[inline(always)]
    pub fn dp_test_mode(&self) -> DpTestModeR {
        DpTestModeR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - When &lt;7:6> ==10, test analog blocks: 000-- disable analog test 001: enable ch0 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 010: enable ch1 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 011: enable ch2 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 100: enable ch3 analog test mux &lt;1:0>(00/01/10/11)-- ->(avdd10_pre_drv,avdd10_p2s, vddu_p2s,avss) 101: enable pll analog test mux &lt;1:0>(00/01/10/11)--->(vdd10_cln,v1p45_v2i, avdd18,vco_ctrl) 110: enable charge pump regulator analog test mux &lt;1:0> (00/01/10/11)---->(v1v_regu, vregu_out,v0.5_ref,null) 111: test band gap output When&lt;7:6>==11, test digital blocks: 000-- disable digital test 001: enable pll digital test mux &lt;1:0>--(00/01/10/11)--->pll_ref,vco_fb, vss,vss. 010: test charge pump regulator OSC clock."]
    #[inline(always)]
    #[must_use]
    pub fn dp_test_bits_1(&mut self) -> DpTestBits1W<DpTestSpec> {
        DpTestBits1W::new(self, 3)
    }
    #[doc = "Bits 6:7 - 00&amp;01: test disables dc_tp/atesto/dtesto output hiz. 10: atesto test enable 11: dtesto test enable"]
    #[inline(always)]
    #[must_use]
    pub fn dp_test_mode(&mut self) -> DpTestModeW<DpTestSpec> {
        DpTestModeW::new(self, 6)
    }
}
#[doc = "Test mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpTestSpec;
impl crate::RegisterSpec for DpTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_test::R`](R) reader structure"]
impl crate::Readable for DpTestSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_test::W`](W) writer structure"]
impl crate::Writable for DpTestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DP_TEST to value 0"]
impl crate::Resettable for DpTestSpec {
    const RESET_VALUE: u32 = 0;
}
